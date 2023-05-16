use crate::{
    materials::Unlit,
    surfaces::{Sphere, Surface},
};
use core::f32;
use glam::{vec3, vec4, Mat3, Vec2, Vec3, Vec4Swizzles};
use pixel_renderer::{
    app::{Callbacks, Config},
    cmd::{canvas, keyboard, media},
    Context, KeyCode,
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

// const WIDTH: u32 = 1920;
// const HEIGHT: u32 = 1080;

const FOCAL_LENGTH: f32 = HEIGHT as f32 / 2.0;

const SURFACE_DISTANCE: f32 = 0.0001;
const EPSILON: f32 = SURFACE_DISTANCE / 10.0; // should be smaller than surface distance
const MAX_DISTANCE: f32 = 50.0;
const MAX_STEPS: u32 = 100;

const CAMERA_MOVE_SPEED: f32 = 2.0;
const CAMERA_ROTATE_SPEED: f32 = 1.0;

const ANTI_ALIASING: bool = true;
const DIFFUSE: bool = true;
const DISTANCE_FOG: bool = true;
const SHADOWS: Shadows = Shadows::Soft(16.0);
// const SHADOWS: Shadows = Shadows::None;
// const SHADOWS: Shadows = Shadows::Hard;
const SHADOWS_FIRST_STEP: f32 = 0.1;

enum Shadows {
    None,
    Hard,
    Soft(f32),
}

pub const AMBIENT_LIGHT: f32 = 0.1;

pub const RED: Vec3 = vec3(1.0, 0.0, 0.0);
pub const GREEN: Vec3 = vec3(0.0, 1.0, 0.0);
pub const BLUE: Vec3 = vec3(0.0, 0.0, 1.0);
pub const WHITE: Vec3 = vec3(1.0, 1.0, 1.0);
pub const YELLOW: Vec3 = vec3(1.0, 1.0, 0.0);
pub const PINK: Vec3 = vec3(1.0, 0.5, 0.5);

// const DEFAULT_MATERIAL: Box<dyn Material> = Box::new(Flat::new(PINK));

/// Holds state needed for ray marcher
pub struct Raymarcher {
    surfaces: Vec<Box<dyn Surface>>,
    camera_pos: Vec3,
    camera_rot: f32,
    light_pos: Vec3,
    debug_light: Sphere,
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
    pub fn new(surfaces: Vec<Box<dyn Surface>>, light_pos: Vec3) -> Self {
        let camera_pos = Vec3::new(0.0, 0.0, -5.0);
        let camera_rot = 0.0;
        let default_material = Box::new(Unlit::new(WHITE));
        let debug_light = Sphere::new(light_pos, 0.1, default_material);
        Self {
            surfaces,
            camera_pos,
            camera_rot,
            light_pos,
            debug_light,
        }
    }

    fn input(&mut self, ctx: &Context, dt: f32) {
        let rot_mat = Mat3::from_rotation_y(self.camera_rot);
        let rot_mat = rot_mat.to_cols_array_2d();
        let right = vec3(rot_mat[0][0], rot_mat[0][1], rot_mat[0][2]).normalize();
        let _up = vec3(rot_mat[1][0], rot_mat[1][1], rot_mat[1][2]).normalize();
        let forward = vec3(rot_mat[2][0], rot_mat[2][1], rot_mat[2][2]).normalize();

        // Camera
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

        // Lights
        if keyboard::key_pressed(ctx, KeyCode::Up) {
            self.light_pos.z += CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::Down) {
            self.light_pos.z -= CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::Right) {
            self.light_pos.x += CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::Left) {
            self.light_pos.x -= CAMERA_MOVE_SPEED * dt;
        }
        self.debug_light.center = self.light_pos;

        // Media
        if keyboard::key_just_pressed(ctx, KeyCode::Space) {
            let path = "outputs/25.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }
    }

    fn get_screen_pos(&self, x: u32, y: u32, offset: Vec2) -> Vec3 {
        vec3(
            x as f32 - WIDTH as f32 / 2.0 + offset.x,
            -(y as f32 - HEIGHT as f32 / 2.0) + offset.y,
            FOCAL_LENGTH,
        )
    }

    // Sample middle of pixel
    fn single_sample_draw(&self, x: u32, y: u32) -> Vec3 {
        let rot_mat = Mat3::from_rotation_y(self.camera_rot);
        // Sample single point
        let screen_pos = self.get_screen_pos(x, y, Vec2::ZERO);
        let dir = (rot_mat * screen_pos).normalize();
        self.raymarch_color(self.camera_pos, dir)
    }

    // Anti aliasing, sample 4 close points
    fn anti_alias_draw(&self, x: u32, y: u32) -> Vec3 {
        let mut color = Vec3::ZERO;
        let rot_mat = Mat3::from_rotation_y(self.camera_rot);

        let e = vec4(0.125, -0.125, 0.375, -0.375);
        for offset in [e.xz(), e.yw(), e.wx(), e.zy()] {
            let screen_pos = self.get_screen_pos(x, y, offset);
            let dir = (rot_mat * screen_pos).normalize();
            color += self.raymarch_color(self.camera_pos, dir);
        }
        color / 4.0
    }

    fn draw(&mut self, ctx: &mut Context) {
        canvas::clear_screen(ctx);

        for y in 0..canvas::height(ctx) {
            for x in 0..canvas::width(ctx) {
                // Anti aliasing
                let color = if ANTI_ALIASING {
                    self.anti_alias_draw(x, y)
                } else {
                    self.single_sample_draw(x, y)
                };

                // Distance fog
                canvas::write_pixel_f32(ctx, x, y, &color.to_array());
            }
        }
    }

    fn raymarch_color(&self, ray_origin: Vec3, ray_dir: Vec3) -> Vec3 {
        let t = self.raymarch_distance(ray_origin, ray_dir);
        if t < MAX_DISTANCE {
            let pos = ray_origin + ray_dir * t;
            return self.hit(ray_dir, pos);
        }
        self.miss()
    }

    fn raymarch_distance(&self, ray_origin: Vec3, ray_dir: Vec3) -> f32 {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let pos = ray_origin + ray_dir * t;
            let res = self.closest_sdf(pos).unwrap();
            let dist = res.0;

            if dist < SURFACE_DISTANCE {
                return t;
                // return self.hit(ray_dir, pos);
            }

            t += dist;
            if t >= MAX_DISTANCE {
                break;
            }
        }
        t
    }

    fn soft_shadow(&self, ray_origin: Vec3, ray_dir: Vec3, k: f32) -> f32 {
        let mut t = 0.0;
        let mut shadow: f32 = 1.0;
        for _ in 0..MAX_STEPS {
            let pos = ray_origin + ray_dir * t;
            let dist = self.closest_sdf(pos).unwrap().0;

            if dist < SURFACE_DISTANCE {
                return 0.0;
            }

            shadow = shadow.min(k * dist / t);
            t += dist;
            if t >= MAX_DISTANCE {
                break;
            }
        }
        shadow
    }

    fn hit(&self, rd: Vec3, pos: Vec3) -> Vec3 {
        let normal = self.normal(pos).normalize();
        let light_dir = (self.light_pos - pos).normalize();
        let relfeced_dir = reflect(-light_dir, normal);
        let view_dir = -rd.normalize();

        // Color
        let (_, material) = self.closest_sdf(pos).unwrap();
        let mut color = material.color(rd, pos, normal, self.light_pos);

        // Phon shading model
        let ambient = 0.1;
        let specular = relfeced_dir.dot(view_dir).clamp(0.0, 1.0).powf(10.0);
        let diffuse = (light_dir.dot(normal).clamp(0.0, 1.0)).clamp(0.0, 1.0);
        let fresnel = (0.1 * (1.0 + rd.dot(normal)).powf(3.0)).max(0.0);

        // Fog
        let distance_surface = (self.camera_pos - pos).length();
        let fog = 1.0 - distance_surface / MAX_DISTANCE;

        // Shadows
        let shadows = match SHADOWS {
            Shadows::Hard => {
                let light_dist = (self.light_pos - pos).length();
                let dist = self.raymarch_distance(pos + light_dir * 0.01, light_dir);
                if dist < light_dist {
                    0.0
                } else {
                    1.0
                }
            }
            Shadows::Soft(_) => {
                self.soft_shadow(pos + light_dir * SHADOWS_FIRST_STEP, light_dir, 16.0)
            }
            Shadows::None => 1.0,
        };

        // Apply shading
        // let a = fresnel;
        // color = vec3(a, a, a);
        // color = normal;
        // color = Vec3::ONE * (ambient);
        // color *= (diffuse * fog * shadows) + ambient;
        // color *= (diffuse + ambient + specular) * shadows * fog;
        color *= (ambient + fresnel) + (specular + diffuse) * shadows;
        color *= fog;

        // Gamma correction
        color = color.powf(0.4545);

        color
    }

    fn miss(&self) -> Vec3 {
        Vec3::ZERO
        // vec3(0.0, 8.0, 8.0)
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        let center = self.closest_sdf(pos).unwrap().0;
        let x = self.closest_sdf(pos + vec3(EPSILON, 0.0, 0.0)).unwrap().0;
        let y = self.closest_sdf(pos + vec3(0.0, EPSILON, 0.0)).unwrap().0;
        let z = self.closest_sdf(pos + vec3(0.0, 0.0, EPSILON)).unwrap().0;
        (vec3(x, y, z) - center) / EPSILON
    }

    fn closest_sdf(&self, pos: Vec3) -> Option<(f32, &dyn Surface)> {
        let mut closest: Option<(f32, &dyn Surface)> = None;
        for surface in self.surfaces.iter() {
            let res = surface.sdf(pos);
            match closest {
                Some(c) if res >= c.0 => {}
                _ => closest = Some((res, surface.as_ref())),
            }
        }

        // DEBUG Light
        // let light_sphere_res = self.debug_light.sdf(pos);
        // match closest {
        //     Some(c) if light_sphere_res >= c.0 => {}
        //     _ => closest = Some((light_sphere_res, &self.debug_light)),
        // }

        closest
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * normal.dot(incident) * normal
}

// struct HitInfo {
//     distance: f32,
//     object_index: usize,
// }
