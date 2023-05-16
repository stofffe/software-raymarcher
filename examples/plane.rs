use glam::vec3;
use raymarching::{
    materials::{Textured, Unlit},
    raymarcher::{Raymarcher, RED},
    surfaces::{Plane, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        Box::new(Plane::new(vec3(0.0, 1.0, 0.0), -2.0, Box::new(Textured::new("assets/checkerboard.jpeg")))),
        Box::new(Sphere::new(vec3(0.0,0.0,0.0), 1.0, Box::new(Unlit::new(RED)))),
        // Box::new(Plane::new(vec3(0.0, -1.0, 0.0), -2.0, Box::new(Unlit::new(WHITE)))),
        // Box::new(BoxExact::new(vec3(1.0,1.0,1.0), Box::new(Unlit::new(GREEN))))
        // Box::new(Plane::new(vec3(0.0,-1.0,0.0), -1.0, Box::new(Shaded::new(Box::new(Unlit::new(BLUE))))))
    ];
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let app = Raymarcher::new(surfaces, light_pos);
    pixel_renderer::app::run(app)
}
