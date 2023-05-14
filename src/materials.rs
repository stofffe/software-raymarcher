use glam::Vec3;

use crate::raymarcher::INDIRECT_LIGHT;

pub trait Material {
    // TODO lights
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

/// Material that outputs a flat color not affected by light
pub struct Unlit {
    color: Vec3,
}

impl Unlit {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Unlit {
    fn color(&self, _ray: Vec3, _pos: Vec3, _normal: Vec3, _light_pos: Vec3) -> Vec3 {
        self.color
    }
}

/// Material that outputs the normal as a color
pub struct Normal;

impl Material for Normal {
    fn color(&self, _ray: Vec3, _pos: Vec3, normal: Vec3, _light_pos: Vec3) -> Vec3 {
        normal
    }
}

// Material that outputs a shaded color
// Uses phong shading
pub struct PhongShaded {
    color: Vec3,
}

impl PhongShaded {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for PhongShaded {
    fn color(&self, _ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let light_dir = pos - light_pos;
        let light = Vec3::dot(normal.normalize(), light_dir.normalize()).max(0.0);
        self.color * (light + INDIRECT_LIGHT)
    }
}
