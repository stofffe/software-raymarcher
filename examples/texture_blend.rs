use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::Textured,
    raymarcher::Raymarcher,
    surfaces::{plane, smooth_union, sphere, translation, SurfaceList},
};

fn main() {
    let checkerboard_mat = Arc::new(Textured::new("assets/checkerboard.jpeg"));
    let dirt_mat = Arc::new(Textured::new("assets/dirt.jpeg"));
    let grass_mat = Arc::new(Textured::new("assets/grass.jpeg"));
    let surfaces: SurfaceList = Arc::new(vec![
        smooth_union(
            translation(vec3(2.0, -1.0, 0.0), sphere(1.0, dirt_mat)),
            translation(vec3(0.0, -1.0, 0.0), sphere(1.0, grass_mat)),
            1.0,
        ),
        plane(vec3(0.0, 1.0, 0.0), -3.0, checkerboard_mat),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
