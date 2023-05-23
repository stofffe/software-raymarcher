use std::sync::Arc;

use glam::{vec3, Vec3};
use raymarching::{
    materials::{Unlit, GREEN, RED, WHITE, YELLOW},
    raymarcher::Raymarcher,
    surfaces::{Sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![
        Arc::new(Sphere::new(Vec3::ZERO, 1.0, Arc::new(Unlit::new(RED)))),
        Arc::new(Sphere::new(
            vec3(2.0, 0.0, 0.0),
            1.0,
            Arc::new(Unlit::new(WHITE)),
        )),
        Arc::new(Sphere::new(
            vec3(4.0, -3.0, 3.0),
            1.0,
            Arc::new(Unlit::new(GREEN)),
        )),
        Arc::new(Sphere::new(
            vec3(-2.0, 2.0, 2.0),
            0.2,
            Arc::new(Unlit::new(YELLOW)),
        )),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
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
