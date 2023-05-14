use glam::Vec3;
use raymarching::{
    raymarcher::{Raymarcher, RED},
    surfaces::{Sphere, Surface},
};

fn main() {
    let surfaces: Vec<Box<dyn Surface>> = vec![Box::new(Sphere::new(Vec3::ZERO, 1.0, RED))];
    let app = Raymarcher::new(surfaces);
    pixel_renderer::app::run(app)
}

// Box::new(SmoothUnion::new(
//     Box::new(Sphere::new(vec3(-1.0, 0.0, 0.0), 1.0)),
//     Box::new(Sphere::new(vec3(1.0, 0.0, 0.0), 1.0)),
//     1.0,
// )),
// Object::new(
//     Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0)),
//     Box::new(Flat::new(RED)),
// ),
// Object::new(
//     Box::new(Sphere::new(vec3(1.0, 1.0, -2.0), 1.0)),
//     Box::new(Normal),
// ),
// Object::new(
//     Box::new(Plane::new(vec3(1.0, -1.0, 0.0), -2.0)),
//     Box::new(Flat::new(BLUE)),
// ),
// Box::new(SmoothUnion::new(
//     Box::new(SmoothUnion::new(
//         Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0, BLUE)),
//         Box::new(Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0, RED)),
//         1.0,
//     )),
//     Box::new(Sphere::new(vec3(-2.0, -1.0, 0.0), 1.0, GREEN)),
//     1.0,
// )),
// Box::new(Intersection::new(
//     Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0, BLUE)),
//     Box::new(Sphere::new(vec3(-0.5, 0.5, 0.0), 1.0, RED)),
// )),
// Box::new(SmoothUnion::new(
//     Box::new(SmoothUnion::new(
//         Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0, BLUE)),
//         Box::new(Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0, RED)),
//         1.0,
//     )),
//     Box::new(Sphere::new(vec3(-2.0, -1.0, 0.0), 1.0, GREEN)),
//     1.0,
// )),
