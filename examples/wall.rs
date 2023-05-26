use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Textured, Unlit, BLUE, GREEN, RED, WHITE, YELLOW},
    raymarcher::Raymarcher,
    surfaces::{plane, smooth_union, sphere, translation, SurfaceList},
};

fn main() {
    let brick_wall_mat = Arc::new(Textured::new("assets/brick_wall.jpeg"));
    let dirt_mat = Arc::new(Textured::new("assets/dirt.jpeg"));
    let surfaces: SurfaceList = Arc::new(vec![
        // Walls and floor
        smooth_union(
            smooth_union(
                plane(vec3(1.0, 0.0, -1.0), -2.0, brick_wall_mat.clone()),
                plane(vec3(-1.0, 0.0, -1.0), -2.0, brick_wall_mat),
                0.05,
            ),
            plane(vec3(0.0, 1.0, 0.0), -1.0, dirt_mat),
            0.1,
        ),
        translation(vec3(0.0, 1.0, -1.0), sphere(1.0, Arc::new(Unlit::new(RED)))),
        translation(
            vec3(-0.8, 3.0, -1.0),
            sphere(0.8, Arc::new(Unlit::new(GREEN))),
        ),
        translation(
            vec3(0.8, 2.5, -1.8),
            sphere(0.5, Arc::new(Unlit::new(BLUE))),
        ),
        smooth_union(
            translation(
                vec3(0.2, 4.5, -2.0),
                sphere(0.5, Arc::new(Unlit::new(YELLOW))),
            ),
            translation(
                vec3(0.8, 3.5, -2.0),
                sphere(0.3, Arc::new(Unlit::new(WHITE))),
            ),
            1.0,
        ),
    ]);
    let light_pos = vec3(2.0, 2.0, -3.0);
    let camera_pos = vec3(0.0, 3.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
