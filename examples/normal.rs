use std::sync::Arc;

use glam::vec3;
use software_raymarcher::{
    materials::Normal,
    raymarcher::Raymarcher,
    surfaces::{sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![sphere(1.0, Arc::new(Normal))]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -3.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
