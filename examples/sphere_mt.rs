use glam::vec3;
use raymarching::raymarcher_mt::Raymarcher;

fn main() {
    let camera_pos = vec3(0.0, 0.0, -3.0);
    let light_pos = vec3(1.0, 2.0, -4.0);
    let raymarcher = Raymarcher::new(camera_pos, light_pos);
    pixel_renderer::app::run(raymarcher);
}
