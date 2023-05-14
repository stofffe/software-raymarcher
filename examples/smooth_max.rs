use glam::vec3;
use raymarching::{
    materials::Lit,
    raymarcher::{Raymarcher, BLUE, GREEN, RED},
    surfaces::{SmoothUnion, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![Box::new(SmoothUnion::new(
        Box::new(SmoothUnion::new(
            Box::new(Sphere::new( vec3(0.0, 0.0, 0.0), 1.0, Box::new(Lit::new(BLUE)),)),
            Box::new(Sphere::new( vec3(-2.0, 1.0, 0.0), 1.0, Box::new(Lit::new(RED)),)),
            1.0,
        )),
        Box::new(Sphere::new( vec3(-2.0, -1.0, 0.0), 1.0, Box::new(Lit::new(GREEN)),)),
        1.0,
    ))];
    let light_dir = vec3(1.0, 1.0, 1.0);
    let app = Raymarcher::new(surfaces, light_dir);
    pixel_renderer::app::run(app)
}
