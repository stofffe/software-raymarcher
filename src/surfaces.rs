use glam::{vec3, vec4, Vec3, Vec4, Vec4Swizzles};

use crate::materials::Material;

// TODO implement blending for non smooth union, subtraction and intersection

/// Represents a surface defined by a SDF
pub trait Surface {
    fn sdf(&self, pos: Vec3) -> Vec4;
}

// Surface representing a sphere defined by position and radius
pub struct Sphere {
    pos: Vec3,
    radius: f32,
    color: Vec3,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32, color: Vec3) -> Self {
        Self { pos, radius, color }
    }
}

impl Surface for Sphere {
    fn sdf(&self, pos: Vec3) -> Vec4 {
        let distance = Vec3::distance(pos, self.pos) - self.radius;
        vec4(self.color.x, self.color.y, self.color.z, distance)
    }
}

// /// Surface representing a plane defined by a normal
// /// height defines distance moved along normal
// pub struct Plane {
//     normal: Vec3,
//     height: f32,
// }
//
// impl Plane {
//     pub fn new(normal: Vec3, height: f32) -> Self {
//         let normal = normal.normalize();
//         Self { normal, height }
//     }
// }
//
// impl Surface for Plane {
//     fn sdf(&self, pos: Vec3) -> f32 {
//         Vec3::dot(pos, self.normal) - self.height
//     }
// }

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
    fn sdf(&self, pos: Vec3) -> Vec4 {
        let surf1 = self.surface1.sdf(pos);
        let surf2 = self.surface2.sdf(pos);
        let dist1 = surf1.w;
        let dist2 = surf2.w;
        let col1 = surf1.xyz();
        let col2 = surf2.xyz();

        if dist2 < dist1 {
            vec4(col2.x, col2.y, col2.z, dist2)
        } else {
            vec4(col1.x, col1.y, col1.z, dist1)
        }
    }
}

/// Surface representing subtraction of two surfaces
pub struct Subtraction {
    surface1: Box<dyn Surface>,
    surface2: Box<dyn Surface>,
}

impl Subtraction {
    pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>) -> Self {
        Self { surface1, surface2 }
    }
}

impl Surface for Subtraction {
    fn sdf(&self, pos: Vec3) -> Vec4 {
        let surf1 = self.surface1.sdf(pos);
        let surf2 = self.surface2.sdf(pos);
        let dist1 = surf1.w;
        let dist2 = surf2.w;
        let col1 = surf1.xyz();
        let col2 = surf2.xyz();

        if -dist2 > dist1 {
            vec4(col2.x, col2.y, col2.z, -dist2)
        } else {
            vec4(col1.x, col1.y, col1.z, dist1)
        }
    }
}

/// Surface representing subtraction of two surfaces
pub struct Intersection {
    surface1: Box<dyn Surface>,
    surface2: Box<dyn Surface>,
}

impl Intersection {
    pub fn new(surface1: Box<dyn Surface>, surface2: Box<dyn Surface>) -> Self {
        Self { surface1, surface2 }
    }
}

impl Surface for Intersection {
    fn sdf(&self, pos: Vec3) -> Vec4 {
        let surf1 = self.surface1.sdf(pos);
        let surf2 = self.surface2.sdf(pos);
        let dist1 = surf1.w;
        let dist2 = surf2.w;
        let col1 = surf1.xyz();
        let col2 = surf2.xyz();

        if dist2 > dist1 {
            vec4(col2.x, col2.y, col2.z, dist2)
        } else {
            vec4(col1.x, col1.y, col1.z, dist1)
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
    fn sdf(&self, pos: Vec3) -> Vec4 {
        let surf1 = self.surface1.sdf(pos);
        let surf2 = self.surface2.sdf(pos);
        let dist1 = surf1.w;
        let dist2 = surf2.w;
        let col1 = surf1.xyz();
        let col2 = surf2.xyz();

        // Distance
        let h = f32::max(self.k - f32::abs(dist1 - dist2), 0.0);
        let distance = f32::min(dist1, dist2) - h * h * 0.25 / self.k;

        // Color
        let p = f32::clamp(0.5 + 0.5 * (dist1 - dist2) / self.k, 0.0, 1.0);
        let color = interpolate_vec3(col1, col2, p);

        vec4(color.x, color.y, color.z, distance)
    }
    // let light = 0.1;
    // let color = vec3(p + light, p + light, p + light);
}

/// Interpolates two vec3 with p [0,1]
fn interpolate_vec3(a: Vec3, b: Vec3, p: f32) -> Vec3 {
    a * (1.0 - p) + b * p
}
