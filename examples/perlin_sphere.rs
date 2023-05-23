use std::sync::Arc;

use glam::{vec3, Vec3};
use raymarching::{
    materials::{Unlit, RED},
    raymarcher::Raymarcher,
    surfaces::{PerlinSphere, PertrubedSphere, SurfaceList},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: SurfaceList = Arc::new(vec![
        // Arc::new(PertrubedSphere::new(Vec3::ZERO, 10.0, 1.0, 1.0, Arc::new(Unlit::new(RED)))),
        Arc::new(PerlinSphere::new(Vec3::ZERO, 10.0, 0.5, Arc::new(Unlit::new(RED)))),
    ]);
    let light_pos = vec3(-15.0, 20.0, -30.0);
    let camera_pos = vec3(-8.0, 0.0, -15.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
