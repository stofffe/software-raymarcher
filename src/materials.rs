use glam::Vec3;

pub trait Material {
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3) -> Vec3;
}

/// Flat color not affected by light
pub struct Flat {
    color: Vec3,
}

impl Flat {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Flat {
    fn color(&self, _ray: Vec3, _pos: Vec3, _normal: Vec3) -> Vec3 {
        self.color
    }
}

/// Material that outputs the normal as a color
pub struct Normal;

impl Material for Normal {
    fn color(&self, _ray: Vec3, _pos: Vec3, normal: Vec3) -> Vec3 {
        normal
    }
}
