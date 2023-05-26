use std::{f32::consts::PI, sync::Arc};

use glam::{vec3, Quat, Vec3};
use raymarching::{
    materials::{Unlit, RED, WHITE},
    raymarcher::Raymarcher,
    surfaces::{
        BoxExact, Rotation, Scale, Sphere, SurfaceList, Translation, TranslationRotationScale,
    },
};

fn main() {
    let translation = vec3(2.0, 1.0, 0.0);
    let translation2 = vec3(-2.0, 1.0, 0.0);
    let rotation = Quat::from_rotation_z(PI / 4.0);
    let rotation2 = Quat::from_rotation_z(-PI / 4.0);
    let scale = 0.5;
    let shape = Arc::new(BoxExact::new(
        vec3(1.0, 2.0, 3.0),
        Arc::new(Unlit::new(RED)),
    ));
    let surfaces: SurfaceList = Arc::new(vec![
        Arc::new(Translation::new(
            translation2,
            Arc::new(Rotation::new(
                rotation2,
                Arc::new(Scale::new(scale, shape.clone())),
            )),
        )),
        Arc::new(TranslationRotationScale::new(
            shape,
            translation,
            rotation,
            scale,
        )),
        // Debug
        // Arc::new(Sphere::new(
        //     vec3(0.0, 0.0, 0.0),
        //     0.1,
        //     Arc::new(Unlit::new(WHITE)),
        // )),
        // Arc::new(Sphere::new(
        //     vec3(translation.x, 0.0, 0.0),
        //     0.1,
        //     Arc::new(Unlit::new(WHITE)),
        // )),
        // Arc::new(Sphere::new(
        //     vec3(translation2.x, 0.0, 0.0),
        //     0.1,
        //     Arc::new(Unlit::new(WHITE)),
        // )),
        // Arc::new(Sphere::new(
        //     vec3(0.0, translation.y, 0.0),
        //     0.1,
        //     Arc::new(Unlit::new(WHITE)),
        // )),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 0.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
