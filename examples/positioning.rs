use std::{f32::consts::PI, sync::Arc};

use glam::{vec3, Quat};
use raymarching::{
    materials::{Unlit, RED},
    raymarcher::Raymarcher,
    surfaces::{exact_box, rotation, scale, translation, translation_rotation_scale, SurfaceList},
};

fn main() {
    let translation1 = vec3(2.0, 1.0, 0.0);
    let translation2 = vec3(-2.0, 1.0, 0.0);
    let rotation1 = Quat::from_rotation_z(PI / 4.0);
    let rotation2 = Quat::from_rotation_z(-PI / 4.0);
    let scale1 = 0.3;
    let scale2 = 0.5;
    let shape = exact_box(vec3(1.0, 2.0, 3.0), Arc::new(Unlit::new(RED)));
    let surfaces: SurfaceList = Arc::new(vec![
        translation(
            translation2,
            rotation(rotation2, scale(scale2, shape.clone())),
        ),
        translation_rotation_scale(translation1, rotation1, scale1, shape),
    ]);
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 1.0, -5.0);
    let app = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(app)
}
