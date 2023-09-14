use auto_ops::*;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[macro_export]
macro_rules! vec3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        Vec3::new($x, $y, $z)
    }};
}
pub(crate) use vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_gl(&self) -> [f32; 4] {
        [self.x, self.y, self.z, 0.0]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vec(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::new(-a.x, -a.y, -a.z) });

impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });

impl_op_ex!(/|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });

impl_op_ex!(+|a: f32, b: &Vec3| -> Vec3 { Vec3::new(a + b.x, a + b.y, a + b.z) });
impl_op_ex!(+|a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x + b, a.y + b, a.z + b) });

impl_op_ex!(-|a: f32, b: &Vec3| -> Vec3 { Vec3::new(a - b.x, a - b.y, a - b.z) });
impl_op_ex!(-|a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x - b, a.y - b, a.z - b) });

impl_op_ex!(*|a: f32, b: &Vec3| -> Vec3 { Vec3::new(a * b.x, a * b.y, a * b.z) });
impl_op_ex!(*|a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });

impl_op_ex!(/|a: f32, b: &Vec3| -> Vec3 { Vec3::new(a / b.x, a / b.y, a / b.z) });
impl_op_ex!(/|a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });
