#version 420 core

#define RENDER_DISTANCE 10000

#define LAMBERTIAN 0
#define METAL 1
#define GLASS 2

struct Camera {
    vec4 origin;
    vec4 p00;
    vec4 du;
    vec4 dv;
    uint width;
    uint height;
    uint sample_size;
    uint max_bounces;
};

struct Ray
{
    vec3 origin;
    vec3 direction;
};

struct HitRecord
{
    float t;
    vec3  p;
    vec3  normal;
    vec4 mat_p; // r, g, b, mat_param
    uint mat;
};

struct Sphere {
    vec4 p; // x, y, z, radius
    vec4 mat_p; // r, g, b, mat_param
    uint mat;
};

layout(origin_upper_left) in vec4 gl_FragCoord;

layout(std140, binding = 0) uniform Scene {
    Camera cam;
    Sphere world[5];
    uint world_size;
};

out vec4 Color;

uint pcg_state = 0;

uint pcg() {
  pcg_state = pcg_state * uint(747796405) + uint(2891336453);
  uint word = ((pcg_state >> ((pcg_state >> uint(28)) + uint(4))) ^ pcg_state) * uint(277803737);
  return (word >> uint(22)) ^ word;
}

void pcg_init()
{
    pcg_state = uint(gl_FragCoord.x);
    pcg_state = pcg() + uint(gl_FragCoord.y);
}

float rand()
{
    return float(pcg()) / float(0xffffffffu);
}

vec3 rand_on_unit_sphere()
{
    return normalize(vec3(rand(), rand(), rand()));
}

Ray get_ray()
{
    Ray ray;
    ray.origin = cam.origin.xyz;

    vec3 p_center = cam.p00.xyz + gl_FragCoord.x * cam.du.xyz + gl_FragCoord.y * cam.dv.xyz;
    vec3 s = p_center + (- 0.5 + rand()) * cam.du.xyz + (- 0.5 + rand()) * cam.dv.xyz;
    ray.direction = s - cam.origin.xyz;

    return ray;
}

bool hit_sphere(Sphere sphere, Ray ray, float t_min, float t_max, out HitRecord rec)
{
    vec3 oc = ray.origin - sphere.p.xyz;
    float a = dot(ray.direction, ray.direction);
    float half_b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.p.w * sphere.p.w;

    float discriminant = half_b * half_b - a * c;
    if (discriminant < 0.0)
    {
        return false;
    }

    float root = (-half_b - sqrt(discriminant)) / a;
    if (root < t_min || root > t_max)
    {
        root = (-half_b + sqrt(discriminant)) / a;
        if (root < t_min || root > t_max)
        {
            return false;
        }
    }

    rec.t = root;
    rec.p = ray.origin + rec.t * ray.direction;
    rec.normal = (rec.p - sphere.p.xyz) / sphere.p.w;
    rec.mat_p = sphere.mat_p;
    rec.mat = sphere.mat;

    return true;
}

bool hit_world(Ray ray, float t_min, float t_max, out HitRecord rec)
{
        HitRecord temp_rec;
        bool hit_anything = false;
        float closest_so_far = t_max;

        for (int i = 0; i < world_size; i++)
        {
            if (hit_sphere(world[i], ray, t_min, closest_so_far, temp_rec))
            {
                hit_anything   = true;
                closest_so_far = temp_rec.t;
                rec            = temp_rec;
            }
        }

        return hit_anything;
}

float reflectance(float cosine, float ref_idx)
{
    float r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    return r0 * r0 + (1.0 - r0 * r0) * pow((1.0 - cosine), 5.0);
}

vec3 refract(vec3 v, vec3 n, float refraction_ratio)
{
    v = normalize(v);
    float cos_theta = min(dot(-v, n), 1.0);
    vec3 r_out_perp = refraction_ratio * (v + n * cos_theta);
    vec3 r_out_parallel = -abs(sqrt((1.0 - dot(r_out_perp, r_out_perp)))) * n;
    return r_out_perp + r_out_parallel;
}

bool scatter(HitRecord rec, Ray ray_in, out Ray ray_out, out vec3 attenuation)
{
    if(rec.mat == LAMBERTIAN)
    {
        vec3 target = rec.p + rec.normal + rand_on_unit_sphere();
        if (target.x < 1.0e-7 && target.y < 1.0e-7 && target.z < 1.0e-7) {
            target = rec.normal;
        }

        ray_out.origin = rec.p;
        ray_out.direction = target - rec.p;

        attenuation = rec.mat_p.rgb;

        return true;
    }
    else if(rec.mat == METAL)
    {
        float fuzz = rec.mat_p.w;

        vec3 reflected = reflect(normalize(ray_in.direction), rec.normal);

        ray_out.origin = rec.p;
        ray_out.direction = reflected + fuzz * rand_on_unit_sphere();

        attenuation = rec.mat_p.rgb;

        return (dot(ray_out.direction, rec.normal) > 0.0);
    }
    else if(rec.mat == GLASS)
    {
        vec3 normal;
        float refraction_ratio;
        float ref_idx = rec.mat_p.w;

        if (dot(ray_in.direction, rec.normal) < 0.0)
        {
            normal = rec.normal;
            refraction_ratio = 1.0f / ref_idx;
        }
        else
        {
            normal = -rec.normal;
            refraction_ratio = ref_idx;
        }

        float cos_theta = min(-dot(normalize(ray_in.direction), normal), 1.0);
        float sin_theta = sqrt(1.0 - cos_theta * cos_theta);
        bool cannot_refract = refraction_ratio * sin_theta > 1.0;

        if (cannot_refract || reflectance(cos_theta, refraction_ratio) > rand())
        {
            ray_out.direction = reflect(ray_in.direction, rec.normal);
        }
        else
        {
            ray_out.direction = refract(ray_in.direction, normal, refraction_ratio);
        }

        ray_out.origin = rec.p;
        attenuation = vec3(1.0, 1.0, 1.0);
        return true;
    }

    return false;
}

vec3 sky_color(Ray ray) {
    float a = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
}

vec3 ray_color(Ray ray)
{
    HitRecord rec;

    vec3 col = vec3(1.0, 1.0, 1.0);

    for(int i = 0; i < cam.max_bounces; i++)
    {
        if (hit_world(ray, 0.001, RENDER_DISTANCE, rec))
        {
            Ray ray_out;
            vec3 attenuation;

            if (scatter(rec, ray, ray_out, attenuation))
            {
                ray.origin = ray_out.origin;
                ray.direction = ray_out.direction;
                col *= attenuation;
            }
            else
            {
                col = vec3(0.0, 0.0, 0.0);
                break;
            }
        }
        else
        {
            col *= sky_color(ray);
            break;
        }
    }

    return col;
}

void main()
{
    pcg_init();

    vec3 col = vec3(0.0, 0.0, 0.0);

    for (int s = 0; s < cam.sample_size; s++)
    {
        Ray r = get_ray();
        col += ray_color(r);
    }

    col /= float(cam.sample_size);

    Color = vec4(sqrt(col.r), sqrt(col.g), sqrt(col.b), 1.0);
}