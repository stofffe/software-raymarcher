use glam::{vec3, vec4, Mat3, Vec2, Vec3, Vec4Swizzles};
use rayon::prelude::*;

use pixel_renderer::{
    app::{Callbacks, Config},
    cmd::{canvas, keyboard, media},
    Context, KeyCode,
};

use crate::surfaces::{Surface, SurfaceList};

enum Shadows {
    None,
    Hard,
    Soft(f32),
}
enum Threading {
    Single,
    ChunkMut(),
    LineChunkMut(u32),
}

enum Antialiasing {
    None,
    AAx4,
}

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
// const WIDTH: u32 = 1920;
// const HEIGHT: u32 = 1080;
const FOCAL_LENGTH: f32 = HEIGHT as f32 / 2.0;

const MAX_STEPS: u32 = 1000;
const MAX_DISTANCE: f32 = 100.0;
const SURFACE_DISTANCE: f32 = 0.0001;
const EPSILON: f32 = SURFACE_DISTANCE / 10.0; // should be smaller than surface distance
const SHADOW_STEP_DISTANCE: f32 = 0.005;

const CAMERA_MOVE_SPEED: f32 = 2.0;
const CAMERA_ROTATE_SPEED: f32 = 1.0;

// const ANTI_ALIASING: Antialiasing = Antialiasing::AAx4;
const ANTI_ALIASING: Antialiasing = Antialiasing::None;

// const THREADING: Threading = Threading::Single;
// const THREADING: Threading = Threading::ChunkMut();
const THREADING: Threading = Threading::LineChunkMut(512);

const SHADOWS: Shadows = Shadows::Soft(16.0);
// const SHADOWS: Shadows = Shadows::Hard;
// const SHADOWS: Shadows = Shadows::None;

pub struct Raymarcher {
    surfaces: SurfaceList,
    camera_pos: Vec3,
    camera_yaw: f32,
    light_pos: Vec3,
    total_dt: f32,
    total_frames: u32,
    skips: i32,
}

impl Callbacks for Raymarcher {
    fn config(&self) -> pixel_renderer::app::Config {
        Config {
            canvas_width: WIDTH,
            canvas_height: HEIGHT,
            resizeable: true,
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) -> bool {
        self.input(ctx, dt);
        self.draw(ctx);

        self.skips -= 1;
        if self.skips <= 0 {
            self.total_dt += dt;
            self.total_frames += 1;
        }

        if self.total_frames == 100 || self.total_frames == 50 || self.total_frames == 25 {
            println!(
                "avg dt for {}: {}",
                self.total_frames,
                self.total_dt / self.total_frames as f32
            );
        }

        println!("dt: {}", dt);

        false
    }
}

impl Raymarcher {
    pub fn new(surfaces: SurfaceList, camera_pos: Vec3, light_pos: Vec3) -> Self {
        Self {
            surfaces,
            camera_pos,
            camera_yaw: 0.0,
            light_pos,
            total_dt: 0.0,
            total_frames: 0,
            skips: 10,
        }
    }

    fn input(&mut self, ctx: &Context, dt: f32) {
        // Camera
        let rot_mat = Mat3::from_rotation_y(self.camera_yaw);
        let rot_mat = rot_mat.to_cols_array_2d();
        let right = vec3(rot_mat[0][0], rot_mat[0][1], rot_mat[0][2]).normalize();
        let up = vec3(rot_mat[1][0], rot_mat[1][1], rot_mat[1][2]).normalize();
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
        if keyboard::key_pressed(ctx, KeyCode::X) {
            self.camera_pos += up * CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::Z) {
            self.camera_pos -= up * CAMERA_MOVE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::Q) {
            self.camera_yaw -= CAMERA_ROTATE_SPEED * dt;
        }
        if keyboard::key_pressed(ctx, KeyCode::E) {
            self.camera_yaw += CAMERA_ROTATE_SPEED * dt;
        }

        // Light
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

        if keyboard::key_just_pressed(ctx, KeyCode::Space) {
            let path = "outputs/32.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("exported screenshot to {}", path);
        }

        // println!("camera pos {}", self.camera_pos);
    }

    fn draw(&mut self, ctx: &mut Context) {
        canvas::clear_screen(ctx);
        let camera_pos = self.camera_pos;
        let light_pos = self.light_pos;
        let rot_mat = Mat3::from_rotation_y(self.camera_yaw);

        match THREADING {
            Threading::Single => {
                draw_single_threaded(ctx, camera_pos, light_pos, rot_mat, self.surfaces.clone())
            }
            Threading::ChunkMut() => draw_multi_threaded_chunkmut(
                ctx,
                camera_pos,
                light_pos,
                rot_mat,
                self.surfaces.clone(),
            ),
            Threading::LineChunkMut(size) => draw_custom_multi_line_chunkmut(
                ctx,
                camera_pos,
                light_pos,
                rot_mat,
                size,
                &self.surfaces,
            ),
        }
    }
}

fn get_screen_pos(x: u32, y: u32, offset: Vec2) -> Vec3 {
    vec3(
        x as f32 - WIDTH as f32 / 2.0 + offset.x,
        -(y as f32 - HEIGHT as f32 / 2.0) + offset.y,
        FOCAL_LENGTH,
    )
}

fn draw_pixel_aax4(
    x: u32,
    y: u32,
    camera_pos: Vec3,
    rot_mat: Mat3,
    light_pos: Vec3,
    surfaces: &[Surface],
) -> Vec3 {
    let mut color = Vec3::ZERO;

    let e = vec4(0.125, -0.125, 0.375, -0.375);
    for offset in [e.xz(), e.yw(), e.wx(), e.zy()] {
        let screen_pos = get_screen_pos(x, y, offset);
        let dir = (rot_mat * screen_pos).normalize();
        color += raymarch_color(camera_pos, dir, light_pos, surfaces);
    }
    color / 4.0
}

fn draw_pixel_simple(
    x: u32,
    y: u32,
    camera_pos: Vec3,
    rot_mat: Mat3,
    light_pos: Vec3,
    surfaces: &[Surface],
) -> Vec3 {
    let screen_pos = get_screen_pos(x, y, Vec2::ZERO);
    let dir = (rot_mat * screen_pos).normalize();
    raymarch_color(camera_pos, dir, light_pos, surfaces)
}

fn raymarch_color(ro: Vec3, rd: Vec3, light_pos: Vec3, surfaces: &[Surface]) -> Vec3 {
    let dist = raymarch(ro, rd, surfaces);
    if dist < MAX_DISTANCE {
        let pos = ro + rd * dist;
        hit(pos, rd, light_pos, ro, surfaces)
    } else {
        miss()
    }
}

fn raymarch(ro: Vec3, rd: Vec3, surfaces: &[Surface]) -> f32 {
    let mut t = 0.0;
    for _ in 0..MAX_STEPS {
        let pos = ro + rd * t;
        let dist = closest_dist(pos, surfaces);

        if dist.abs() < SURFACE_DISTANCE && dist.is_sign_positive() {
            break;
        }

        t += dist;
        if t >= MAX_DISTANCE {
            break;
        }
    }
    // println!("DISTANCE: MAX STEPS REACHED");
    t
}

fn hit(pos: Vec3, rd: Vec3, light_pos: Vec3, camera_pos: Vec3, surfaces: &[Surface]) -> Vec3 {
    let normal = normal(pos, surfaces);
    let light_dir = (light_pos - pos).normalize();
    let relfeced_dir = reflect(-light_dir, normal);
    let view_dir = -rd.normalize();

    // Phong shading model
    let ambient = 0.1;
    let specular = relfeced_dir.dot(view_dir).clamp(0.0, 1.0).powf(10.0);
    let diffuse = 0.9 * (light_dir.dot(normal).clamp(0.0, 1.0)).clamp(0.0, 1.0);
    let fresnel = (0.1 * (1.0 + rd.dot(normal)).powf(3.0)).max(0.0);

    // Fog
    let distance_surface = (camera_pos - pos).length();
    let fog = 1.0 - distance_surface / MAX_DISTANCE;

    // Shadows
    #[rustfmt::skip]
    let shadow = match SHADOWS {
        Shadows::Hard => hard_shadow(pos, light_pos, surfaces),
        Shadows::Soft(k) => soft_shadow(pos,light_pos, k, surfaces),
        Shadows::None => 1.0,
    };

    // Combine
    let mut color = closest_color(rd, pos, normal, light_pos, surfaces);
    color *= (ambient + fresnel) + (specular + diffuse) * shadow;
    color *= fog;
    // let mut color = Vec3::ONE * (specular + ambient);

    // Gamma correction
    color = color.powf(0.4545);

    color
}

fn miss() -> Vec3 {
    vec3(0.0, 0.0, 0.0)
}

fn hard_shadow(surface_pos: Vec3, light_pos: Vec3, surfaces: &[Surface]) -> f32 {
    let light_dir = (light_pos - surface_pos).normalize();
    let light_dist = light_pos.distance(surface_pos);
    let start_pos = surface_pos + light_dir * SHADOW_STEP_DISTANCE; // start a little outside

    let dist = raymarch(start_pos, light_dir, surfaces);

    if dist < light_dist {
        0.0
    } else {
        1.0
    }
}

fn soft_shadow(surface_pos: Vec3, light_pos: Vec3, k: f32, surfaces: &[Surface]) -> f32 {
    let light_dir = (light_pos - surface_pos).normalize();
    let light_dist = light_pos.distance(surface_pos);

    let mut t = SHADOW_STEP_DISTANCE; // start a little outside
    let mut shadow: f32 = 1.0;
    for _ in 0..MAX_STEPS {
        // If we pass the light return white
        if t >= light_dist {
            return shadow;
        }

        let pos = surface_pos + light_dir * t;
        let dist = closest_dist(pos, surfaces);

        // If we hit something before reaching the light return black
        if dist.abs() < SURFACE_DISTANCE {
            return 0.0;
        }

        // Calculate shadow and t
        shadow = shadow.min(k * dist / t);
        t += dist;
    }
    println!("SOFT SHADOW: REACHED MAX STEPS");
    1.0
}

fn normal(pos: Vec3, surfaces: &[Surface]) -> Vec3 {
    let center = closest_dist(pos, surfaces);
    let diff = vec3(
        closest_dist(pos + vec3(EPSILON, 0.0, 0.0), surfaces) - center,
        closest_dist(pos + vec3(0.0, EPSILON, 0.0), surfaces) - center,
        closest_dist(pos + vec3(0.0, 0.0, EPSILON), surfaces) - center,
    );
    // grad / EPSILON
    diff.normalize()
    // let x = closest_dist(pos + vec3(EPSILON, 0.0, 0.0), surfaces);
    // let y = closest_dist(pos + vec3(0.0, EPSILON, 0.0), surfaces);
    // let z = closest_dist(pos + vec3(0.0, 0.0, EPSILON), surfaces);
    // (vec3(x, y, z) - center) / EPSILON
}

fn closest_color(
    ray: Vec3,
    pos: Vec3,
    normal: Vec3,
    light_pos: Vec3,
    surfaces: &[Surface],
) -> Vec3 {
    let mut closest = MAX_DISTANCE;
    let mut closest_surf: Option<&Surface> = None;
    for surface in surfaces.iter() {
        let res = surface.sdf(pos);
        if res < closest {
            closest = res;
            closest_surf = Some(surface);
        }
    }
    if let Some(closest_surf) = closest_surf {
        closest_surf.color(ray, pos, normal, light_pos)
    } else {
        vec3(0.0, 0.0, 0.0)
    }
}

fn closest_dist(pos: Vec3, surfaces: &[Surface]) -> f32 {
    // return 0.0;
    let mut closest = MAX_DISTANCE;
    for surface in surfaces.iter() {
        let res = surface.sdf(pos);
        if res < closest {
            closest = res;
        }
    }
    closest
    // closest.0

    // Perlin
    // let perlin_center = vec3(0.0, 0.0, 0.0);
    // let perlin_radius = 1.0;
    // let intensity = 0.3;

    // let offset = perlin.get([pos.x as f64, pos.y as f64, pos.z as f64]) as f32;
    // let perlin = pos.distance(perlin_center) - perlin_radius + offset * intensity;

    // Sphere
    // let sphere_center = vec3(0.5, 0.0, 0.0);
    // let sphere_radius = 1.0;
    // let sphere = pos.distance(sphere_center) - sphere_radius;
    //
    // // Plane
    // let plane_normal = vec3(0.0, 1.0, 0.0);
    // let h = -2.0;
    // let plane = pos.dot(plane_normal) - h;
    //
    // // Subtraction
    // // let subtraction = (-sphere).max(perlin);
    //
    // // subtraction
    // // sphere
    // // plane
    //
    // sphere.min(plane)

    // pos.distance(sphere_center) - sphere_radius
}
fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * normal.dot(incident) * normal
}

fn draw_single_threaded(
    ctx: &mut Context,
    camera_pos: Vec3,
    light_pos: Vec3,
    rot_mat: Mat3,
    surfaces: SurfaceList,
) {
    let pixels = 0..(WIDTH * HEIGHT);
    pixels.into_iter().for_each(|i| {
        let (x, y) = (i % WIDTH, i / WIDTH);
        let color = match ANTI_ALIASING {
            Antialiasing::None => {
                draw_pixel_simple(x, y, camera_pos, rot_mat, light_pos, &surfaces)
            }
            Antialiasing::AAx4 => draw_pixel_aax4(x, y, camera_pos, rot_mat, light_pos, &surfaces),
        };
        canvas::write_pixel_f32(ctx, x, y, &color.to_array());
    });
}

fn draw_multi_threaded_chunkmut(
    ctx: &mut Context,
    camera_pos: Vec3,
    light_pos: Vec3,
    rot_mat: Mat3,
    surfaces: SurfaceList,
) {
    let pixels = canvas::pixel_ref(ctx);
    pixels.par_chunks_mut(4).enumerate().for_each(|(i, rgba)| {
        let (x, y) = (i as u32 % WIDTH, i as u32 / WIDTH);
        let color = match ANTI_ALIASING {
            Antialiasing::None => {
                draw_pixel_simple(x, y, camera_pos, rot_mat, light_pos, &surfaces)
            }
            Antialiasing::AAx4 => draw_pixel_aax4(x, y, camera_pos, rot_mat, light_pos, &surfaces),
        }
        .clamp(Vec3::ZERO, Vec3::ONE);
        rgba[0] = (color.x * 255.0) as u8;
        rgba[1] = (color.y * 255.0) as u8;
        rgba[2] = (color.z * 255.0) as u8;
    });
}

fn draw_custom_multi_line_chunkmut(
    ctx: &mut Context,
    camera_pos: Vec3,
    light_pos: Vec3,
    rot_mat: Mat3,
    size: u32,
    surfaces: &SurfaceList,
) {
    // let size = 32;
    let pixels = canvas::pixel_ref(ctx);
    pixels
        .par_chunks_mut(size as usize * 4)
        .enumerate()
        .for_each(|(i, line)| {
            for (j, rgba) in line.chunks_mut(4).enumerate() {
                let index = i as u32 * size + j as u32;
                let (x, y) = (index % WIDTH, index / WIDTH);
                let color = match ANTI_ALIASING {
                    Antialiasing::None => {
                        draw_pixel_simple(x, y, camera_pos, rot_mat, light_pos, surfaces)
                    }
                    Antialiasing::AAx4 => {
                        draw_pixel_aax4(x, y, camera_pos, rot_mat, light_pos, surfaces)
                    }
                }
                .clamp(Vec3::ZERO, Vec3::ONE);
                rgba[0] = (color.x * 255.0) as u8;
                rgba[1] = (color.y * 255.0) as u8;
                rgba[2] = (color.z * 255.0) as u8;
                // println!("({x}, {y}) color: {:?}, rgba; {:?}", color, rgba);
            }
        });
}
