use std::sync::Arc;

use glam::{vec3, Vec3};
use raymarching::{
    materials::{Unlit, BLUE, RED},
    raymarcher::Raymarcher,
    surfaces::{
        BoxExact, Intersection, Material, SmoothIntersection, SmoothSubtraction, SmoothUnion,
        Sphere, Subtraction, Surface, SurfaceList, Translation, Union,
    },
};

fn main() {
    let x_space = 5.0;
    let z_space = 5.0;
    let red_mat = Arc::new(Unlit::new(RED));
    let blue_mat = Arc::new(Unlit::new(BLUE));
    let surfaces: SurfaceList = Arc::new(vec![
        // Union
        translated(
            vec3(0.0, 0.0, z_space),
            union(
                translated(
                    vec3(0.0, 0.0, 0.0),
                    exact_box(vec3(2.0, 1.0, 2.0), red_mat.clone()),
                ),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat.clone())),
            ),
        ),
        translated(
            vec3(0.0, 0.0, 0.0),
            smooth_union(
                translated(
                    vec3(0.0, 0.0, 0.0),
                    exact_box(vec3(2.0, 1.0, 2.0), red_mat.clone()),
                ),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat.clone())),
                0.5,
            ),
        ),
        // Subtraction
        translated(
            vec3(x_space, 0.0, z_space),
            subtraction(
                translated(
                    vec3(0.0, 0.0, 0.0),
                    exact_box(vec3(2.0, 1.0, 2.0), red_mat.clone()),
                ),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat.clone())),
            ),
        ),
        translated(
            vec3(x_space, 0.0, 0.0),
            smooth_subtraction(
                translated(
                    vec3(0.0, 0.0, 0.0),
                    exact_box(vec3(2.0, 1.0, 2.0), red_mat.clone()),
                ),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat.clone())),
                0.5,
            ),
        ),
        // Intersection
        translated(
            vec3(2.0 * x_space, 0.0, z_space),
            intersection(
                translated(
                    vec3(0.0, 0.0, 0.0),
                    exact_box(vec3(2.0, 1.0, 2.0), red_mat.clone()),
                ),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat.clone())),
            ),
        ),
        translated(
            vec3(2.0 * x_space, 0.0, 0.0),
            smooth_intersection(
                translated(vec3(0.0, 0.0, 0.0), exact_box(vec3(2.0, 1.0, 2.0), red_mat)),
                translated(vec3(0.0, 1.0, 0.0), sphere(1.5, blue_mat)),
                0.5,
            ),
        ),
    ]);
    let light_pos = vec3(4.0, 2.0, -5.0);
    let camera_pos = vec3(x_space, 6.0, -10.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}

fn translated(translation: Vec3, surface: Surface) -> Surface {
    Arc::new(Translation::new(translation, surface))
}

fn sphere(radius: f32, material: Material) -> Surface {
    Arc::new(Sphere::new(radius, material))
}
fn exact_box(b: Vec3, material: Material) -> Surface {
    Arc::new(BoxExact::new(b, material))
}

fn union(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Union::new(surface1, surface2))
}
fn intersection(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Intersection::new(surface1, surface2))
}
fn subtraction(surface1: Surface, surface2: Surface) -> Surface {
    Arc::new(Subtraction::new(surface1, surface2))
}

fn smooth_union(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothUnion::new(surface1, surface2, blend_factor))
}
fn smooth_subtraction(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothSubtraction::new(surface1, surface2, blend_factor))
}
fn smooth_intersection(surface1: Surface, surface2: Surface, blend_factor: f32) -> Surface {
    Arc::new(SmoothIntersection::new(surface1, surface2, blend_factor))
}
