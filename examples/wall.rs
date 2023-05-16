use glam::vec3;
use raymarching::{
    materials::{Textured, Unlit},
    raymarcher::{Raymarcher, BLUE, GREEN, RED, WHITE, YELLOW},
    surfaces::{Plane, SmoothUnion, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        // Walls and floor
        Box::new(SmoothUnion::new(
            Box::new(SmoothUnion::new(
                Box::new(Plane::new( vec3(1.0, 0.0, -1.0), -2.0, Box::new(Textured::new("assets/brick_wall.jpeg")),)),
                Box::new(Plane::new( vec3(-1.0, 0.0, -1.0), -2.0, Box::new(Textured::new("assets/brick_wall.jpeg")),)),
                0.05,
            )),
            Box::new(Plane::new( vec3(0.0, 1.0, 0.0), -1.0, Box::new(Textured::new("assets/dirt.jpeg")),)),
            0.1,
        )),
                 // Box::new(Plane::new( vec3(1.0, 0.0, -1.0), -2.0, Box::new(Textured::new("assets/brick_wall.jpeg")),)),
                 // Box::new(Plane::new( vec3(-1.0, 0.0, -1.0), -2.0, Box::new(Textured::new("assets/brick_wall.jpeg")),)),
            // Box::new(Plane::new( vec3(0.0, 1.0, 0.0), -1.0, Box::new(Textured::new("assets/dirt.jpeg")),)),
        Box::new(Sphere::new(vec3(0.0, 1.0, -1.0), 1.0, Box::new(Unlit::new(RED)))),
        Box::new(Sphere::new(vec3(-0.8, 3.0, -1.0), 0.8, Box::new(Unlit::new(GREEN)))),
        Box::new(Sphere::new(vec3(0.8, 2.5, -1.8), 0.5, Box::new(Unlit::new(BLUE)))),
        Box::new(SmoothUnion::new(
            Box::new(Sphere::new(vec3(0.2, 4.5, -2.0), 0.5, Box::new(Unlit::new(YELLOW)))),
            Box::new(Sphere::new(vec3(0.8, 3.5, -2.0), 0.3, Box::new(Unlit::new(WHITE)))),
            1.0,
        )),
        // Box::new(Sphere::new(light_pos, 0.2, Box::new(Unlit::new(WHITE)))),
    ];
    let light_pos = vec3(2.0, 2.0, -3.0);
    let camera_pos = vec3(0.0, 3.0, -5.0);
    let app = Raymarcher::new(surfaces, light_pos, camera_pos);
    pixel_renderer::app::run(app)
}
