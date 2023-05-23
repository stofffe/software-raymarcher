use std::sync::Arc;

use glam::vec3;
use raymarching::{
    materials::{Normal, Unlit, BLUE, RED},
    raymarcher::Raymarcher,
    surfaces::{SmoothUnion, Sphere, SurfaceList},
};

fn main() {
    let surfaces: SurfaceList = Arc::new(vec![Arc::new(SmoothUnion::new(
        Arc::new(SmoothUnion::new(
            Arc::new(Sphere::new(
                vec3(0.0, 0.0, 0.0),
                1.0,
                Arc::new(Unlit::new(BLUE)),
            )),
            Arc::new(Sphere::new(
                vec3(-2.0, 1.0, 0.0),
                1.0,
                Arc::new(Unlit::new(RED)),
            )),
            1.0,
        )),
        Arc::new(Sphere::new(vec3(-2.0, -1.0, 0.0), 1.0, Arc::new(Normal))),
        1.0,
    ))]);
    let light_pos = vec3(1.0, 1.0, -1.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
