use crate::surfaces::Surface;
use core::f32;
use glam::{vec3, vec4, Mat3, Vec3, Vec4, Vec4Swizzles};
use pixel_renderer::{
    app::{Callbacks, Config},
    cmd::{canvas, keyboard, media},
    Context, KeyCode,
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const FOCAL_LENGTH: f32 = HEIGHT as f32 / 2.0;

const EPSILON: f32 = 0.00001; // should be smaller than surface distance
const SURFACE_DISTANCE: f32 = 0.0001;
const MAX_DISTANCE: f32 = 10.0;
const MAX_STEPS: u32 = 100;

const CAMERA_MOVE_SPEED: f32 = 2.0;
const CAMERA_ROTATE_SPEED: f32 = 1.0;

pub const RED: Vec3 = vec3(1.0, 0.0, 0.0);
pub const GREEN: Vec3 = vec3(0.0, 1.0, 0.0);
pub const BLUE: Vec3 = vec3(0.0, 0.0, 1.0);

/// Holds state needed for ray marcher
pub struct Raymarcher {
    surfaces: Vec<Box<dyn Surface>>,
    camera_pos: Vec3,
    camera_rot: f32,
    light_dir: Vec3,
}

impl Callbacks for Raymarcher {
    fn update(&mut self, ctx: &mut Context, dt: f32) -> bool {
        // println!("{dt}");

        self.input(ctx, dt);
        self.draw(ctx);

        false
    }

    fn config(&self) -> Config {
        Config {
            canvas_width: WIDTH,
            canvas_height: HEIGHT,
            resizeable: true,
            ..Default::default()
        }
    }
}

impl Raymarcher {
    pub fn new(surfaces: Vec<Box<dyn Surface>>, light_dir: Vec3) -> Self {
        let camera_pos = Vec3::new(0.0, 0.0, -5.0);
        let camera_rot = 0.0;
        Self {
            surfaces,
            camera_pos,
            camera_rot,
            light_dir,
        }
    }

    fn input(&mut self, ctx: &Context, dt: f32) {
        let rot_mat = Mat3::from_rotation_y(self.camera_rot);
        let rot_mat = rot_mat.to_cols_array_2d();
        let right = vec3(rot_mat[0][0], rot_mat[0][1], rot_mat[0][2]).normalize();
        let _up = vec3(rot_mat[1][0], rot_mat[1][1], rot_mat[1][2]).normalize();
        let forward = vec3(rot_mat[2][0], rot_mat[2][1], rot_mat[2][2]).normalize();

        if keyboard::key_pressed(ctx, KeyCode::W) {
            self.camera_pos += forward * CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::S) {
            self.camera_pos -= forward * CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::A) {
            self.camera_pos -= right * CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::D) {
            self.camera_pos += right * CAMERA_MOVE_SPEED * dt;
        }

        if keyboard::key_pressed(ctx, KeyCode::Q) {
            self.camera_rot -= CAMERA_ROTATE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::E) {
            self.camera_rot += CAMERA_ROTATE_SPEED * dt;
        }

        if keyboard::key_just_pressed(ctx, KeyCode::Space) {
            let path = "outputs/13.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }
    }

    fn draw(&self, ctx: &mut Context) {
        canvas::clear_screen(ctx);

        let rot_mat = Mat3::from_rotation_y(self.camera_rot);
        for y in 0..canvas::height(ctx) {
            for x in 0..canvas::width(ctx) {
                // Get uv coordinates and direction
                let screen_pos = vec3(
                    x as f32 - WIDTH as f32 / 2.0,
                    y as f32 - HEIGHT as f32 / 2.0,
                    FOCAL_LENGTH,
                );
                let dir = (rot_mat * screen_pos).normalize();
                let color = self.raymarch(self.camera_pos, dir);
                canvas::write_pixel_f32(ctx, x, y, &color.to_array());
            }
        }
    }

    fn raymarch(&self, ray_origin: Vec3, ray_dir: Vec3) -> Vec3 {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let pos = ray_origin + ray_dir * t;
            let res = self.closest_sdf(pos);
            if res.w < SURFACE_DISTANCE {
                return self.hit(ray_dir, pos);
            }

            t += res.w;
            if t >= MAX_DISTANCE {
                break;
            }
        }
        self.miss()
    }

    fn hit(&self, rd: Vec3, pos: Vec3) -> Vec3 {
        // println!("pos {pos}");
        let res = self.closest_sdf(pos);
        let color = res.xyz();
        let normal = self.normal(pos);
        let light = Vec3::dot(normal.normalize(), self.light_dir.normalize()).max(0.0);
        color * (light + 0.2)
        // normal
        // normal * normal
        // color.xyz()
    }

    fn miss(&self) -> Vec3 {
        Vec3::ZERO
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        let center = self.closest_sdf(pos).w;
        let x = self.closest_sdf(pos - vec3(EPSILON, 0.0, 0.0)).w;
        let y = self.closest_sdf(pos - vec3(0.0, EPSILON, 0.0)).w;
        let z = self.closest_sdf(pos - vec3(0.0, 0.0, EPSILON)).w;
        (vec3(x, y, z) - center) / EPSILON
    }

    fn closest_sdf(&self, pos: Vec3) -> Vec4 {
        let mut closest = vec4(0.0, 0.0, 0.0, MAX_DISTANCE);
        for (i, surface) in self.surfaces.iter().enumerate() {
            let res = surface.sdf(pos);
            if res.w < closest.w {
                closest = res;
            }
        }
        closest
    }
}

// struct HitInfo {
//     distance: f32,
//     object_index: usize,
// }
