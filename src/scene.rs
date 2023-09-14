use crate::vec3::*;

#[repr(u32)]
pub enum Material {
    Lambertian,
    Metal,
    Glass,
}

#[repr(C, align(16))]
struct Sphere {
    _p: [f32; 4],     // x, y, z, radius
    _mat_p: [f32; 4], // r, g, b, mat_param
    _mat: Material,
}

#[repr(C, align(16))]
struct Camera {
    _origin: [f32; 4],
    _p00: [f32; 4],
    _du: [f32; 4],
    _dv: [f32; 4],
    _width: u32,
    _height: u32,
    _sample_size: u32,
    _max_bounces: u32,
}

impl Camera {
    pub fn new(
        img_width: u32,
        img_height: u32,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        sample_size: u32,
        max_bounces: u32,
    ) -> Self {
        let focal_length = (look_from - look_at).length();
        let h = (vfov.to_radians() / 2.0).tan();
        let vp_height = 2.0 * h * focal_length;
        let vp_width = vp_height * (img_width as f32 / img_height as f32);

        let w = (look_from - look_at).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        let vp_u = vp_width * u;
        let vp_v = vp_height * -v;

        let du = vp_u / img_width as f32;
        let dv = vp_v / img_height as f32;

        let viewport_upper_left = look_from - (focal_length * w) - vp_u / 2.0 - vp_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (du + dv);

        Self {
            _width: img_width,
            _height: img_height,
            _origin: look_from.to_gl(),
            _p00: pixel00.to_gl(),
            _du: du.to_gl(),
            _dv: dv.to_gl(),
            _sample_size: sample_size,
            _max_bounces: max_bounces,
        }
    }
}

#[repr(C, align(16))]
pub struct Scene {
    _cam: Camera,
    _world: [Sphere; 5],
    _world_size: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            _cam: Camera::new(
                1600,
                900,
                vec3!(-2.0, 2.0, 1.0),
                vec3!(0.0, 0.0, -1.0),
                vec3!(0.0, 1.0, 0.0),
                60.0,
                100,
                50,
            ),
            _world: [
                Sphere {
                    _p: [0.0, -100.5, -1.0, 100.0],
                    _mat_p: [0.8, 0.8, 0.0, 0.0],
                    _mat: Material::Lambertian,
                },
                Sphere {
                    _p: [0.0, 0.0, -1.0, 0.5],
                    _mat_p: [0.1, 0.2, 0.5, 0.0],
                    _mat: Material::Lambertian,
                },
                Sphere {
                    _p: [-1.0, 0.0, -1.0, 0.5],
                    _mat_p: [0.0, 0.0, 0.0, 1.5],
                    _mat: Material::Glass,
                },
                Sphere {
                    _p: [-1.3, 0.7, 0.0, 0.1],
                    _mat_p: [0.0, 0.0, 0.0, 1.5],
                    _mat: Material::Glass,
                },
                Sphere {
                    _p: [1.0, 0.0, -1.0, 0.5],
                    _mat_p: [0.8, 0.6, 0.2, 0.1],
                    _mat: Material::Metal,
                },
            ],
            _world_size: 5,
        }
    }
}
