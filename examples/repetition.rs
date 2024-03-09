use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Unlit, BLUE},
    raymarcher::Raymarcher,
    surfaces::{infinite_repetition, sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![infinite_repetition(
        vec3(4.0, 4.0, 4.0),
        sphere(1.0, Arc::new(Unlit::new(BLUE))),
    )]);
    let light_pos = vec3(2.0, 2.0, -0.0);
    let camera_pos = vec3(2.0, 2.0, -0.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixelated::run(app)
}
