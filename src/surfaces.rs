use glam::Vec3;

use crate::materials::Material;

/// Represents a surface defined by a SDF
pub trait Surface {
    fn sdf(&self, pos: Vec3) -> f32;
    // fn material(&self) -> &dyn Material;
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

// Surface representing a sphere defined by position and radius
// TODO pos should be represented using translation?
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Surface for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        Vec3::distance(pos, self.center) - self.radius
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

pub struct BoxExact {
    b: Vec3,
    material: Box<dyn Material>,
}

impl BoxExact {
    pub fn new(b: Vec3, material: Box<dyn Material>) -> Self {
        Self { b, material }
    }
}

impl Surface for BoxExact {
    fn sdf(&self, pos: Vec3) -> f32 {
        let q = pos.abs() - self.b;
        q.max(Vec3::ZERO).length() + (q.x.max(q.y.max(q.z))).min(0.0)
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

/// Surface representing a plane defined by ```normal```
/// ```origin_distance``` units from the origin
pub struct Plane {
    normal: Vec3,
    origin_distance: f32,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(normal: Vec3, origin_distance: f32, material: Box<dyn Material>) -> Self {
        let normal = normal.normalize();
        Self {
            normal,
            origin_distance,
            material,
        }
    }
}

impl Surface for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.dot(self.normal) - self.origin_distance
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

/// Surface representing union of two surfaces
pub struct Union {
    surface1: Box<dyn Surface>,
    surface2: Box<dyn Surface>,
}

impl Union {
    pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>) -> Self {
        Self { surface1, surface2 }
    }
}

impl Surface for Union {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        if dist2 < dist1 {
            dist2
        } else {
            dist1
        }
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

/// Surface representing smooth union of two surfaces
/// k is smoothing distance
pub struct SmoothUnion {
    surface1: Box<dyn Surface>,
    surface2: Box<dyn Surface>,
    k: f32,
}

impl SmoothUnion {
    pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>, blend_factor: f32) -> Self {
        Self {
            surface1,
            surface2,
            k: blend_factor,
        }
    }
}

impl Surface for SmoothUnion {
    fn sdf(&self, pos: Vec3) -> f32 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);

        // Distance
        let h = f32::max(self.k - f32::abs(dist1 - dist2), 0.0);
        f32::min(dist1, dist2) - h * h * 0.25 / self.k
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        let dist1 = self.surface1.sdf(pos);
        let dist2 = self.surface2.sdf(pos);
        let p = (0.5 + 0.5 * (dist1 - dist2) / self.k).clamp(0.0, 1.0);

        let color1 = self.surface1.color(ray, pos, normal, light_pos);
        let color2 = self.surface2.color(ray, pos, normal, light_pos);

        interpolate_vec3(color1, color2, p)
    }
}

/// Interpolates two vec3 with p [0,1]
pub fn interpolate_vec3(a: Vec3, b: Vec3, p: f32) -> Vec3 {
    a * (1.0 - p) + b * p
}
// fn sdf(&self, pos: Vec3) -> Vec4 {
//     let surf1 = self.surface1.sdf(pos);
//     let surf2 = self.surface2.sdf(pos);
//     let dist1 = surf1.w;
//     let dist2 = surf2.w;
//     let col1 = surf1.xyz();
//     let col2 = surf2.xyz();
//
//     // Distance
//     let h = f32::max(self.k - f32::abs(dist1 - dist2), 0.0);
//     let distance = f32::min(dist1, dist2) - h * h * 0.25 / self.k;
//
//     // Color
//     let p = f32::clamp(0.5 + 0.5 * (dist1 - dist2) / self.k, 0.0, 1.0);
//     let color = interpolate_vec3(col1, col2, p);
//
//     vec4(color.x, color.y, color.z, distance)
// }

// let light = 0.1;
// let color = vec3(p + light, p + light, p + light);
// }

//
//
// /// Surface that translates a child surface
// pub struct TranslationRotation {
//     translation: Vec3,
//     rotation: Quat,
//     surface: Box<dyn Surface>,
// }
//
// impl TranslationRotation {
//     pub fn new(surface: Box<dyn Surface>, translation: Vec3, rotation: Quat) -> Self {
//         Self {
//             surface,
//             translation,
//             rotation,
//         }
//     }
// }
//
// impl Surface for TranslationRotation {
//     fn sdf(&self, pos: Vec3) -> Vec4 {
//         let new_pos = self.rotation * (self.translation - pos);
//         self.surface.sdf(new_pos)
//     }
//     // let mat = Mat4::from_rotation_translation(self.rotation, self.translation).inverse();
//     // let new_pos = (mat * vec4(pos.x, pos.y, pos.z, 0.0)).xyz();
// }
//
//
// /// Surface representing subtraction of two surfaces
// pub struct Subtraction {
//     surface1: Box<dyn Surface>,
//     surface2: Box<dyn Surface>,
// }
//
// impl Subtraction {
//     pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>) -> Self {
//         Self { surface1, surface2 }
//     }
// }
//
// impl Surface for Subtraction {
//     fn sdf(&self, pos: Vec3) -> Vec4 {
//         let surf1 = self.surface1.sdf(pos);
//         let surf2 = self.surface2.sdf(pos);
//         let dist1 = surf1.w;
//         let dist2 = surf2.w;
//         let col1 = surf1.xyz();
//         let col2 = surf2.xyz();
//
//         if -dist2 > dist1 {
//             vec4(col2.x, col2.y, col2.z, -dist2)
//         } else {
//             vec4(col1.x, col1.y, col1.z, dist1)
//         }
//     }
// }
//
// /// Surface representing subtraction of two surfaces
// pub struct Intersection {
//     surface1: Box<dyn Surface>,
//     surface2: Box<dyn Surface>,
// }
//
// impl Intersection {
//     pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>) -> Self {
//         Self { surface1, surface2 }
//     }
// }
//
// impl Surface for Intersection {
//     fn sdf(&self, pos: Vec3) -> Vec4 {
//         let surf1 = self.surface1.sdf(pos);
//         let surf2 = self.surface2.sdf(pos);
//         let dist1 = surf1.w;
//         let dist2 = surf2.w;
//         let col1 = surf1.xyz();
//         let col2 = surf2.xyz();
//
//         if dist2 > dist1 {
//             vec4(col2.x, col2.y, col2.z, dist2)
//         } else {
//             vec4(col1.x, col1.y, col1.z, dist1)
//         }
//     }
// }
