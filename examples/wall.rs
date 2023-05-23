use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Texture, Textured, Unlit, BLUE, GREEN, RED, WHITE, YELLOW},
    raymarcher::Raymarcher,
    surfaces::{Plane, SmoothUnion, Sphere, SurfaceList},
};

fn main() {
    let brick_wall_mat = Arc::new(Textured::new("assets/brick_wall.jpeg"));
    let dirt_mat = Arc::new(Textured::new("assets/dirt.jpeg"));
    let surfaces: SurfaceList = Arc::new(vec![
        // Walls and floor
        Arc::new(SmoothUnion::new(
            Arc::new(SmoothUnion::new(
                Arc::new(Plane::new(
                    vec3(1.0, 0.0, -1.0),
                    -2.0,
                    brick_wall_mat.clone(),
                )),
                Arc::new(Plane::new(vec3(-1.0, 0.0, -1.0), -2.0, brick_wall_mat)),
                0.05,
            )),
            Arc::new(Plane::new(vec3(0.0, 1.0, 0.0), -1.0, dirt_mat)),
            0.1,
        )),
        Arc::new(Sphere::new(
            vec3(0.0, 1.0, -1.0),
            1.0,
            Arc::new(Unlit::new(RED)),
        )),
        Arc::new(Sphere::new(
            vec3(-0.8, 3.0, -1.0),
            0.8,
            Arc::new(Unlit::new(GREEN)),
        )),
        Arc::new(Sphere::new(
            vec3(0.8, 2.5, -1.8),
            0.5,
            Arc::new(Unlit::new(BLUE)),
        )),
        Arc::new(SmoothUnion::new(
            Arc::new(Sphere::new(
                vec3(0.2, 4.5, -2.0),
                0.5,
                Arc::new(Unlit::new(YELLOW)),
            )),
            Arc::new(Sphere::new(
                vec3(0.8, 3.5, -2.0),
                0.3,
                Arc::new(Unlit::new(WHITE)),
            )),
            1.0,
        )),
        // Box::new(Sphere::new(light_pos, 0.2, Box::new(Unlit::new(WHITE)))),
    ]);
    let light_pos = vec3(2.0, 2.0, -3.0);
    let camera_pos = vec3(0.0, 3.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
