use std::sync::Arc;

use glam::{vec3, Vec3};
use noise::{NoiseFn, Perlin};

use crate::materials::Material;

pub type Surface = Arc<dyn SurfaceTrait>;
pub type SurfaceList = Arc<Vec<Surface>>;

/// Represents a surface defined by a SDF
pub trait SurfaceTrait: Sync + Send {
    fn sdf(&self, pos: Vec3) -> f32;
    // fn material(&self) -> &dyn Material;
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

pub type Mat = Arc<dyn Material + Sync + Send>;

// Surface representing a sphere defined by position and radius
// TODO pos should be represented using translation?
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    material: Mat,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Mat) -> Self {
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

pub struct BoxExact {
    b: Vec3,
    material: Mat,
}

impl BoxExact {
    pub fn new(b: Vec3, material: Mat) -> Self {
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

/// Surface representing a plane defined by ```normal```
/// ```origin_distance``` units from the origin
pub struct Plane {
    normal: Vec3,
    origin_distance: f32,
    material: Mat,
}

impl Plane {
    pub fn new(normal: Vec3, origin_distance: f32, material: Mat) -> Self {
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

pub struct PerlinSphere {
    pub center: Vec3,
    radius: f32,
    material: Mat,
    perlin: Perlin,
    intensity: f32,
}

impl PerlinSphere {
    pub fn new(center: Vec3, radius: f32, intensity: f32, material: Mat) -> Self {
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

pub struct PertrubedSphere {
    pub center: Vec3,
    radius: f32,
    material: Mat,
    intensity: f32,
    phase_shift: f32,
}

impl PertrubedSphere {
    pub fn new(center: Vec3, radius: f32, intensity: f32, phase_shift: f32, material: Mat) -> Self {
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

pub struct Fractal {
    material: Mat,
    power: f32,
}

impl Fractal {
    pub fn new(power: f32, material: Mat) -> Self {
        Self { power, material }
    }
}

const STEPS: usize = 20;

impl SurfaceTrait for Fractal {
    fn sdf(&self, pos: Vec3) -> f32 {
        let mut z = pos;
        let mut dr = 1.0;
        let mut r = 0.0;
        let mut iterations = 0;

        for i in 0..STEPS {
            r = z.length();
            iterations = i;

            if r > 4.0 {
                // let mut dst = 0.5 * r.log2() * r / dr;
                // dst /= iterations as f32 + 1.0;
                // println!("{r} > 4.0 dst: {dst}");
                break;
            }

            let mut phi = (pos.y / pos.x).atan();
            let mut theta = (pos.z / r).acos();
            dr = r.powf(self.power - 1.0) * self.power + 1.0;

            let zr = r.powf(self.power);
            theta *= self.power;
            phi *= self.power;

            z = zr
                * vec3(
                    theta.sin() * phi.cos(),
                    phi.sin() * theta.sin(),
                    theta.cos(),
                );

            z += pos;
        }
        let dst = 0.5 * r.log10() * r / dr;
        // if dst < 0.1 {
        //     println!("SMALLER")
        // }
        dst
    }

    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
        self.material.color(ray, pos, normal, light_pos)
    }
}

/// Interpolates two vec3 with p [0,1]
pub fn interpolate_vec3(a: Vec3, b: Vec3, p: f32) -> Vec3 {
    a * (1.0 - p) + b * p
}
