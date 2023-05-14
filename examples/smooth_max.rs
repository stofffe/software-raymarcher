use glam::vec3;
use raymarching::{
    materials::PhongShaded,
    raymarcher::{Raymarcher, BLUE, GREEN, RED},
    surfaces::{SmoothUnion, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![Box::new(SmoothUnion::new(
        Box::new(SmoothUnion::new(
            Box::new(Sphere::new( vec3(0.0, 0.0, 0.0), 1.0, Box::new(PhongShaded::new(BLUE)),)),
            Box::new(Sphere::new( vec3(-2.0, 1.0, 0.0), 1.0, Box::new(PhongShaded::new(RED)),)),
            1.0,
        )),
        Box::new(Sphere::new( vec3(-2.0, -1.0, 0.0), 1.0,
            Box::new(PhongShaded::new(GREEN)),
        )),
        1.0,
    ))];
    let light_pos = vec3(1.0, 1.0, -1.0);
    let app = Raymarcher::new(surfaces, light_pos);
    pixel_renderer::app::run(app)
}
