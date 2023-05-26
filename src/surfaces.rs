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

pub struct Sphere {
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(radius: f32, material: Material) -> Self {
        Self { radius, material }
    }
}

impl SurfaceTrait for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.length() - self.radius
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        // vec3(1.0, 1.0, 1.0)
        self.material.color(ray, pos, normal, light_pos)
    }
}
pub fn sphere(radius: f32, material: Material) -> Surface {
    Arc::new(Sphere::new(radius, material))
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
pub fn exact_box(b: Vec3, material: Material) -> Surface {
    Arc::new(BoxExact::new(b, material))
}

//
// Plane
//

pub struct Plane {
    normal: Vec3,
    distance_along_normal: f32,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vec3, distance_along_normal: f32, material: Material) -> Self {
        let normal = normal.normalize();
        Self {
            normal,
            distance_along_normal,
            material,
        }
    }
}

impl SurfaceTrait for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.dot(self.normal) - self.distance_along_normal
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}
pub fn plane(normal: Vec3, distance_along_normal: f32, material: Material) -> Surface {
    Arc::new(Plane::new(normal, distance_along_normal, material))
}

//
// Union
//

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
pub fn union(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Union::new(surface1, surface2))
}
//
// Subtraction
//

/// Subtracts surface2 from surface1
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
pub fn subtraction(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Subtraction::new(surface1, surface2))
}
//
// Intersection
//

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
pub fn intersection(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Intersection::new(surface1, surface2))
}

//
// SmoothUnion
//

pub struct SmoothUnion {
    surface1: Arc<dyn SurfaceTrait>,
    surface2: Arc<dyn SurfaceTrait>,
    blend_factor: f32,
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
            blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothUnion {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        let h = (0.5 + 0.5 * (dist2 - dist1) / self.blend_factor).clamp(0.0, 1.0);
        interpolate_f32(dist2, dist1, h) - self.blend_factor * h * (1.0 - h)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);
        let h = (0.5 + 0.5 * (dist1 - dist2) / self.blend_factor).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color1, color2, h)
    }
}

pub fn smooth_union(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothUnion::new(surface1, surface2, blend_factor))
}

//
// Smooth Subtraction
//

/// Subtracts surface2 from surface1
pub struct SmoothSubtraction {
    surface1: Surface,
    surface2: Surface,
    blend_factor: f32,
}

impl SmoothSubtraction {
    pub fn new(surface1: Surface, surface2: Surface, blend_factor: f32) -> Self {
        Self {
            surface1,
            surface2,
            blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothSubtraction {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist2 = self.surface1.sdf(pos);
        let dist1 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 + dist1) / self.blend_factor).clamp(0.0, 1.0);

        interpolate_f32(dist2, -dist1, h) + self.blend_factor * h * (1.0 - h)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 + dist1) / self.blend_factor).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color1, color2, h)
    }
}
pub fn smooth_subtraction(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothSubtraction::new(surface1, surface2, blend_factor))
}

//
// Smooth Intersection
//

pub struct SmoothIntersection {
    surface1: Surface,
    surface2: Surface,
    blend_factor: f32,
}

impl SmoothIntersection {
    pub fn new(surface1: Surface, surface2: Surface, blend_factor: f32) -> Self {
        Self {
            surface1,
            surface2,
            blend_factor,
        }
    }
}

impl SurfaceTrait for SmoothIntersection {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        // Distance
        let h = (0.5 - 0.5 * (dist2 - dist1) / self.blend_factor).clamp(0.0, 1.0);
        interpolate_f32(dist2, dist1, h) + self.blend_factor * h * (1.0 - h)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        let h = (0.5 - 0.5 * (dist2 - dist1) / self.blend_factor).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color2, color1, h)
    }
}
pub fn smooth_intersection(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothIntersection::new(surface1, surface2, blend_factor))
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
pub fn translation(translation: Vec3, surface: Surface) -> Surface {
    Arc::new(Translation::new(translation, surface))
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
pub fn rotation(rotation: Quat, surface: Surface) -> Surface {
    Arc::new(Rotation::new(rotation, surface))
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
pub fn scale(scale: f32, surface: Surface) -> Surface {
    Arc::new(Scale::new(scale, surface))
}

//
// Translation Rotation Scale
//

pub struct TranslationRotationScale {
    surface: Surface,
    translation: Vec3,
    rotation: Quat,
    scale: f32,
}

impl TranslationRotationScale {
    pub fn new(translation: Vec3, rotation: Quat, scale: f32, surface: Surface) -> Self {
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
pub fn translation_rotation_scale(
    translation: Vec3,
    rotation: Quat,
    scale: f32,
    surface: Surface,
) -> Surface {
    Arc::new(TranslationRotationScale::new(
        translation,
        rotation,
        scale,
        surface,
    ))
}

//
// Infinite Repetition
//

pub struct InfiniteRepetition {
    period: Vec3,
    surface: Surface,
}

impl InfiniteRepetition {
    pub fn new(period: Vec3, surface: Surface) -> Self {
        Self { surface, period }
    }
}

fn modulo(x: Vec3, y: Vec3) -> Vec3 {
    x - y * (x / y).floor()
}

impl SurfaceTrait for InfiniteRepetition {
    fn sdf(&self, pos: Vec3) -> f32 {
        let c = self.period;
        let q = modulo(pos + 0.5 * c, c) - 0.5 * c;
        self.surface.sdf(q)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.surface.color(ray, pos, normal, light_pos)
    }
}
pub fn infinite_repetition(period: Vec3, surface: Surface) -> Surface {
    Arc::new(InfiniteRepetition::new(period, surface))
}

//
// Perlin Sphere
//

pub struct PerlinSphere {
    radius: f32,
    material: Material,
    perlin: Perlin,
    intensity: f32,
}

impl PerlinSphere {
    pub fn new(radius: f32, intensity: f32, material: Material) -> Self {
        let perlin = Perlin::new(radius as u32);
        Self {
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
        pos.length() - self.radius + offset * self.intensity
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}
pub fn perlin_sphere(radius: f32, intensity: f32, material: Material) -> Surface {
    Arc::new(PerlinSphere::new(radius, intensity, material))
}

//
// Pertrubed Sphere
//

pub struct PertrubedSphere {
    radius: f32,
    intensity: f32,
    phase_shift: f32,
    material: Material,
}

impl PertrubedSphere {
    pub fn new(radius: f32, intensity: f32, phase_shift: f32, material: Material) -> Self {
        Self {
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
        pos.length() - self.radius + offset
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}
pub fn pertrubed_sphere(
    radius: f32,
    intensity: f32,
    phase_shift: f32,
    material: Material,
) -> Surface {
    Arc::new(PertrubedSphere::new(
        radius,
        intensity,
        phase_shift,
        material,
    ))
}

/// p should be in range [0,1]
pub fn interpolate_vec3(a: Vec3, b: Vec3, p: f32) -> Vec3 {
    a * (1.0 - p) + b * p
}

/// p should be in range [0,1]
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
