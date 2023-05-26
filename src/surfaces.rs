use std::sync::Arc;

use glam::{vec3, Quat, Vec3};
use noise::{NoiseFn, Perlin};

use crate::materials::MaterialTrait;

//
// Type definitions
//

pub type Surface = Arc<dyn SurfaceTrait>;
pub type SurfaceList = Arc<Vec<Surface>>;
pub type Material = Arc<dyn MaterialTrait + Sync + Send>;

/// Represents a surface defined by a SDF
pub trait SurfaceTrait: Sync + Send {
    fn sdf(&self, pos: Vec3) -> f32;
    // fn material(&self) -> &dyn Material;
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

//
// Sphere
//

// Surface representing a sphere defined by position and radius
// TODO pos should be represented using translation?
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl SurfaceTrait for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        Vec3::distance(pos, self.center) - self.radius
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        // vec3(1.0, 1.0, 1.0)
        self.material.color(ray, pos, normal, light_pos)
    }
}

//
// Box Exact
//

pub struct BoxExact {
    b: Vec3,
    material: Material,
}

impl BoxExact {
    pub fn new(b: Vec3, material: Material) -> Self {
        Self { b, material }
    }
}

impl SurfaceTrait for BoxExact {
    fn sdf(&self, pos: Vec3) -> f32 {
        let q = pos.abs() - self.b;
        q.max(Vec3::ZERO).length() + (q.x.max(q.y.max(q.z))).min(0.0)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

//
// Plane
//

/// Surface representing a plane defined by ```normal```
/// ```origin_distance``` units from the origin
pub struct Plane {
    normal: Vec3,
    origin_distance: f32,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vec3, origin_distance: f32, material: Material) -> Self {
        let normal = normal.normalize();
        Self {
            normal,
            origin_distance,
            material,
        }
    }
}

impl SurfaceTrait for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.dot(self.normal) - self.origin_distance
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

//
// Union
//

/// Surface representing union of two surfaces
pub struct Union {
    surface1: Arc<dyn SurfaceTrait>,
    surface2: Arc<dyn SurfaceTrait>,
}

impl Union {
    pub fn new(surface1: Arc<dyn SurfaceTrait>, surface2: Arc<dyn SurfaceTrait>) -> Self {
        Self { surface1, surface2 }
    }
}

impl SurfaceTrait for Union {
    fn sdf(&self, pos: Vec3) -> f32 {
        self.surface1.sdf(pos).min(self.surface2.sdf(pos))
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        if dist2 < dist1 {
            self.surface2.color(ray, pos, normal, light_pos)
        } else {
            self.surface1.color(ray, pos, normal, light_pos)
        }
    }
}
//
// Subtraction
//

/// Surface representing subtraction of two surfaces
/// Surface1 - Surface2
pub struct Subtraction {
    surface1: Surface,
    surface2: Surface,
}

impl Subtraction {
    pub fn new(surface1: Surface, surface2: Surface) -> Self {
        Self { surface1, surface2 }
    }
}

impl SurfaceTrait for Subtraction {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);
        (dist1).max(-dist2)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        if -dist2 > dist1 {
            self.surface2.color(ray, pos, normal, light_pos)
        } else {
            self.surface1.color(ray, pos, normal, light_pos)
        }
    }
}
//
// Intersection
//

/// Surface representing intersection of two surfaces
pub struct Intersection {
    surface1: Surface,
    surface2: Surface,
}

impl Intersection {
    pub fn new(surface1: Surface, surface2: Surface) -> Self {
        Self { surface1, surface2 }
    }
}

impl SurfaceTrait for Intersection {
    fn sdf(&self, pos: Vec3) -> f32 {
        self.surface1.sdf(pos).max(self.surface2.sdf(pos))
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        if dist2 > dist1 {
            self.surface2.color(ray, pos, normal, light_pos)
        } else {
            self.surface1.color(ray, pos, normal, light_pos)
        }
    }
}

//
// SmoothUnion
//

/// Surface representing smooth union of two surfaces
/// k is smoothing distance
pub struct SmoothUnion {
    surface1: Arc<dyn SurfaceTrait>,
    surface2: Arc<dyn SurfaceTrait>,
    k: f32,
}

impl SmoothUnion {
    pub fn new(
        surface1: Arc<dyn SurfaceTrait>,
        surface2: Arc<dyn SurfaceTrait>,
        blend_factor: f32,
    ) -> Self {
        Self {
            surface1,
            surface2,
            k: blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothUnion {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        // Distance
        let h = (0.5 + 0.5 * (dist2 - dist1) / self.k).clamp(0.0, 1.0);
        interpolate_f32(dist2, dist1, h) - self.k * h * (1.0 - h)
        // let h = f32::max(self.k - f32::abs(dist1 - dist2), 0.0);
        // f32::min(dist1, dist2) - h * h * 0.25 / self.k
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);
        let h = (0.5 + 0.5 * (dist1 - dist2) / self.k).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color1, color2, h)
        // let h = (self.k - f32::abs(dist1 - dist2)).max(0.0);
        // vec3(h + 0.01, h + 0.01, h + 0.01);
        // let h = (0.5 + 0.5 * (dist1 - dist2) / self.k).clamp(0.0, 1.0);
        // vec3(h + 0.01, h + 0.01, h + 0.01)
    }
}

//
// Smooth Subtraction
//

/// Surface representing smooth union of two surfaces
/// k is smoothing distance
pub struct SmoothSubtraction {
    surface1: Surface,
    surface2: Surface,
    k: f32,
}

impl SmoothSubtraction {
    pub fn new(surface1: Surface, surface2: Surface, blend_factor: f32) -> Self {
        Self {
            surface1,
            surface2,
            k: blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothSubtraction {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist2 = self.surface1.sdf(pos);
        let dist1 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 + dist1) / self.k).clamp(0.0, 1.0);

        interpolate_f32(dist2, -dist1, h) + self.k * h * (1.0 - h)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 + dist1) / self.k).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color1, color2, h)
    }
}

//
// Smooth Intersection
//

/// Surface representing smooth union of two surfaces
/// k is smoothing distance
pub struct SmoothIntersection {
    surface1: Surface,
    surface2: Surface,
    k: f32,
}

impl SmoothIntersection {
    pub fn new(surface1: Surface, surface2: Surface, blend_factor: f32) -> Self {
        Self {
            surface1,
            surface2,
            k: blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothIntersection {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        // Distance
        let h = (0.5 - 0.5 * (dist2 - dist1) / self.k).clamp(0.0, 1.0);
        interpolate_f32(dist2, dist1, h) + self.k * h * (1.0 - h)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 - dist1) / self.k).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color2, color1, h)
    }
}

//
// Translation
//

pub struct Translation {
    translation: Vec3,
    surface: Surface,
}

impl Translation {
    pub fn new(translation: Vec3, surface: Surface) -> Self {
        Self {
            surface,
            translation,
        }
    }
}

impl SurfaceTrait for Translation {
    fn sdf(&self, pos: Vec3) -> f32 {
        let new_pos = pos - self.translation;
        self.surface.sdf(new_pos)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let new_pos = pos - self.translation;
        self.surface.color(ray, new_pos, normal, light_pos)
    }
}

//
// Rotation
//

pub struct Rotation {
    rotation: Quat,
    surface: Surface,
}

impl Rotation {
    pub fn new(rotation: Quat, surface: Surface) -> Self {
        Self { surface, rotation }
    }
}

impl SurfaceTrait for Rotation {
    fn sdf(&self, pos: Vec3) -> f32 {
        let new_pos = self.rotation * pos;
        self.surface.sdf(new_pos)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.surface.color(ray, pos, normal, light_pos)
    }
}

//
// Scale
//

pub struct Scale {
    scale: f32,
    surface: Surface,
}

impl Scale {
    pub fn new(scale: f32, surface: Surface) -> Self {
        Self { surface, scale }
    }
}

impl SurfaceTrait for Scale {
    fn sdf(&self, pos: Vec3) -> f32 {
        self.surface.sdf(pos / self.scale) * self.scale
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.surface.color(ray, pos, normal, light_pos)
    }
}

//
// TranslationRotationScale
//

pub struct TranslationRotationScale {
    surface: Surface,
    translation: Vec3,
    rotation: Quat,
    scale: f32,
}

impl TranslationRotationScale {
    pub fn new(surface: Surface, translation: Vec3, rotation: Quat, scale: f32) -> Self {
        Self {
            surface,
            translation,
            rotation,
            scale,
        }
    }
}

impl SurfaceTrait for TranslationRotationScale {
    fn sdf(&self, pos: Vec3) -> f32 {
        let new_pos = self.rotation * (pos - self.translation);
        self.surface.sdf(new_pos / self.scale) * self.scale
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.surface.color(ray, pos, normal, light_pos)
    }
}

//
// Repetition
//

pub struct InfRepetition {
    period: Vec3,
    surface: Surface,
}

impl InfRepetition {
    pub fn new(surface: Surface, period: Vec3) -> Self {
        Self { surface, period }
    }
}

fn modulo(x: Vec3, y: Vec3) -> Vec3 {
    x - y * (x / y).floor()
}

impl SurfaceTrait for InfRepetition {
    fn sdf(&self, pos: Vec3) -> f32 {
        let c = self.period;
        let q = modulo(pos + 0.5 * c, c) - 0.5 * c;
        self.surface.sdf(q)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.surface.color(ray, pos, normal, light_pos)
    }
}

//
// Perlin Sphere
//

pub struct PerlinSphere {
    pub center: Vec3,
    radius: f32,
    material: Material,
    perlin: Perlin,
    intensity: f32,
}

impl PerlinSphere {
    pub fn new(center: Vec3, radius: f32, intensity: f32, material: Material) -> Self {
        let perlin = Perlin::new(radius as u32);
        Self {
            center,
            radius,
            material,
            perlin,
            intensity,
        }
    }
}

impl SurfaceTrait for PerlinSphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        let offset = self.perlin.get([pos.x as f64, pos.y as f64, pos.z as f64]) as f32;
        Vec3::distance(pos, self.center) - self.radius + offset * self.intensity
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

//
// Pertrubed Sphere
//

pub struct PertrubedSphere {
    pub center: Vec3,
    radius: f32,
    material: Material,
    intensity: f32,
    phase_shift: f32,
}

impl PertrubedSphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        intensity: f32,
        phase_shift: f32,
        material: Material,
    ) -> Self {
        Self {
            center,
            radius,
            material,
            intensity,
            phase_shift,
        }
    }
}

impl SurfaceTrait for PertrubedSphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        let c = self.intensity;
        let q = self.phase_shift;
        let offset = c * (q + pos.x).sin() * (q + pos.y).sin() * (q + pos.z).sin();
        Vec3::distance(pos, self.center) - self.radius + offset
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

/// Interpolates two vec3 with p [0,1]
pub fn interpolate_vec3(a: Vec3, b: Vec3, p: f32) -> Vec3 {
    a * (1.0 - p) + b * p
}
pub fn interpolate_f32(a: f32, b: f32, p: f32) -> f32 {
    a * (1.0 - p) + b * p
}

// pub struct Fractal {
//     material: Material,
//     power: f32,
// }
//
// impl Fractal {
//     pub fn new(power: f32, material: Material) -> Self {
//         Self { power, material }
//     }
// }
//
// const STEPS: usize = 20;
//
// impl SurfaceTrait for Fractal {
//     fn sdf(&self, pos: Vec3) -> f32 {
//         let mut z = pos;
//         let mut dr = 1.0;
//         let mut r = 0.0;
//         let mut iterations = 0;
//
//         for i in 0..STEPS {
//             r = z.length();
//             iterations = i;
//
//             if r > 4.0 {
//                 // let mut dst = 0.5 * r.log2() * r / dr;
//                 // dst /= iterations as f32 + 1.0;
//                 // println!("{r} > 4.0 dst: {dst}");
//                 break;
//             }
//
//             let mut phi = (pos.y / pos.x).atan();
//             let mut theta = (pos.z / r).acos();
//             dr = r.powf(self.power - 1.0) * self.power + 1.0;
//
//             let zr = r.powf(self.power);
//             theta *= self.power;
//             phi *= self.power;
//
//             z = zr
//                 * vec3(
//                     theta.sin() * phi.cos(),
//                     phi.sin() * theta.sin(),
//                     theta.cos(),
//                 );
//
//             z += pos;
//         }
//         let dst = 0.5 * r.log10() * r / dr;
//         // if dst < 0.1 {
//         //     println!("SMALLER")
//         // }
//         dst
//     }
//
//     fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
//         self.material.color(ray, pos, normal, light_pos)
//     }
// }
