use glam::vec3;
use raymarching::{
    materials::Textured,
    raymarcher::Raymarcher,
    surfaces::{Plane, SmoothUnion, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        Box::new(SmoothUnion::new(
            Box::new(Sphere::new(vec3(2.0,-1.0,0.0), 1.0, Box::new(Textured::new("assets/dirt.jpeg")))),
            Box::new(Sphere::new(vec3(0.0,-1.0,0.0), 1.0, Box::new(Textured::new("assets/grass.jpeg")))),
            1.0,
        )),
        Box::new(Sphere::new(vec3(-3.0,-1.0,0.0), 1.0, Box::new(Textured::new("assets/checkerboard.png").with_blend_sharpness(15.0)))),
        Box::new(Sphere::new(vec3(-3.0,-1.0,-3.0), 1.0, Box::new(Textured::new("assets/dirt.jpeg")))),
        Box::new(Plane::new(vec3(0.0, 1.0, 0.0), -3.0, Box::new(Textured::new("assets/checkerboard.jpeg")))),
    ];
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, light_pos, camera_pos);
    pixel_renderer::app::run(app)
}

