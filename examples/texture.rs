use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::Textured,
    raymarcher::Raymarcher,
    surfaces::{Plane, SmoothUnion, Sphere, SurfaceList},
};

fn main() {
    let checkerboard_mat = Arc::new(Textured::new("assets/checkerboard.jpeg"));
    let dirt_mat = Arc::new(Textured::new("assets/dirt.jpeg"));
    let grass_mat = Arc::new(Textured::new("assets/grass.jpeg"));
    let surfaces: SurfaceList = Arc::new(vec![
        Arc::new(SmoothUnion::new(
            Arc::new(Sphere::new(vec3(2.0, -1.0, 0.0), 1.0, dirt_mat.clone())),
            Arc::new(Sphere::new(vec3(0.0, -1.0, 0.0), 1.0, grass_mat)),
            1.0,
        )),
        Arc::new(Sphere::new(
            vec3(-3.0, -1.0, 0.0),
            1.0,
            checkerboard_mat.clone(),
        )),
        Arc::new(Sphere::new(vec3(-3.0, -1.0, -3.0), 1.0, dirt_mat)),
        Arc::new(Plane::new(vec3(0.0, 1.0, 0.0), -3.0, checkerboard_mat)),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
