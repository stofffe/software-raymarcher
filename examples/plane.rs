use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Textured, Unlit, RED},
    raymarcher::Raymarcher,
    surfaces::{plane, sphere, SurfaceList},
};

fn main() {
    let checkerboard_mat = Arc::new(Textured::new("assets/checkerboard.jpeg"));
    let surfaces: SurfaceList = Arc::new(vec![
        plane(vec3(0.0, 1.0, 0.0), -2.0, checkerboard_mat),
        sphere(1.0, Arc::new(Unlit::new(RED))),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 1.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixelated::run(app)
}
