use std::sync::Arc;

use glam::{vec3, Vec3};
use raymarching::{
    materials::{Unlit, BLUE},
    raymarcher::Raymarcher,
    surfaces::{InfRepetition, Sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![Arc::new(InfRepetition::new(
        Arc::new(Sphere::new(Vec3::ZERO, 1.0, Arc::new(Unlit::new(BLUE)))),
        vec3(4.0, 4.0, 4.0),
    ))]);
    let light_pos = vec3(2.0, 2.0, -0.0);
    let camera_pos = vec3(2.0, 2.0, -0.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
