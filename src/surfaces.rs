use glam::Vec3;

/// Represents a surface defined by a SDF
pub trait Surface {
    fn sdf(&self, pos: Vec3) -> f32;
}

// Surface representing a sphere defined by position and radius
pub struct Sphere {
    pos: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }
}

impl Surface for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        Vec3::distance(pos, self.pos) - self.radius
    }
}

/// Surface representing a plane defined by a normal
/// height defines distance moved along normal
pub struct Plane {
    normal: Vec3,
    height: f32,
}

impl Plane {
    pub fn new(normal: Vec3, height: f32) -> Self {
        let normal = normal.normalize();
        Self { normal, height }
    }
}

impl Surface for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        Vec3::dot(pos, self.normal) - self.height
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
        f32::min(self.surface1.sdf(pos), self.surface2.sdf(pos))
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
    fn sdf(&self, pos: Vec3) -> f32 {
        f32::max(-self.surface1.sdf(pos), self.surface2.sdf(pos))
    }
}

/// Surface representing intersection of two surfaces
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
    fn sdf(&self, pos: Vec3) -> f32 {
        f32::max(self.surface1.sdf(pos), self.surface2.sdf(pos))
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
        let d1 = self.surface1.sdf(pos);
        let d2 = self.surface2.sdf(pos);
        let interpolation = f32::max(self.k - f32::abs(d1 - d2), 0.0);
        f32::min(d1, d2) - interpolation * interpolation * 0.25 / self.k
    }
    // let h = f32::clamp(0.5 + 0.5 * (d2 - d1) / self.blend_factor, 0.0, 1.0);
    // f32::min(self.surface1.sdf(pos), self.surface2.sdf(pos))
}
