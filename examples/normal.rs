use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Normal, Textured, Unlit, RED},
    raymarcher::Raymarcher,
    surfaces::{Plane, Sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![Arc::new(Sphere::new(
        vec3(0.0, 0.0, 0.0),
        1.0,
        Arc::new(Normal),
    ))]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 1.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
