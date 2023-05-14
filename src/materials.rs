use glam::Vec3;

use crate::surfaces::interpolate_vec3;

pub trait Material {
    // TODO lights
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

/// Flat color not affected by light
pub struct Unlit {
    color: Vec3,
}

impl Unlit {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Unlit {
    fn color(&self, _ray: Vec3, _pos: Vec3, _normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.color
    }
}

/// Material that outputs the normal as a color
pub struct Normal;

impl Material for Normal {
    fn color(&self, _ray: Vec3, _pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        normal
    }
}

pub struct Lit {
    color: Vec3,
}

impl Lit {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Lit {
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let light_dir = pos - light_pos;
        let light = Vec3::dot(normal.normalize(), light_dir.normalize()).max(0.0);
        self.color * (light + 0.2)
    }
}

/// p [0,1] and describes
/// Interpolates material color
pub struct CombinedMaterial<'a> {
    material1: &'a dyn Material,
    material2: &'a dyn Material,
    pub p: f32,
}

impl<'a> CombinedMaterial<'a> {
    pub fn new(material1: &'a dyn Material, material2: &'a dyn Material, p: f32) -> Self {
        Self {
            material1,
            material2,
            p,
        }
    }
}

impl<'a> Material for CombinedMaterial<'a> {
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let color1 = self.material1.color(ray, pos, normal, light_pos);
        let color2 = self.material2.color(ray, pos, normal, light_pos);
        interpolate_vec3(color1, color2, self.p)
    }
}
