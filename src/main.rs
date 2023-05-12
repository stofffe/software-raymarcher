use glam::{vec3, Mat3, Vec3};
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

const RED: Vec3 = vec3(1.0, 0.0, 0.0);
const GREEN: Vec3 = vec3(0.0, 1.0, 0.0);
const BLUE: Vec3 = vec3(0.0, 0.0, 1.0);

const CAMERA_MOVE_SPEED: f32 = 2.0;
const CAMERA_ROTATE_SPEED: f32 = 1.0;

/// Holds state needed for ray marcher
struct Raymarcher {
    objects: Vec<Object>,
    camera_pos: Vec3,
    camera_rot: f32,
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
    fn new() -> Self {
        let objects: Vec<Object> = vec![
            Object::new(
                Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0)),
                Box::new(Flat::new(RED)),
            ),
            Object::new(
                Box::new(Sphere::new(vec3(1.0, 1.0, -2.0), 1.0)),
                Box::new(Normal),
            ),
            Object::new(
                Box::new(Plane::new(vec3(1.0, -1.0, 0.0), -2.0)),
                Box::new(Flat::new(BLUE)),
            ),
        ];
        let camera_pos = Vec3::new(0.0, 0.0, -5.0);
        let camera_rot = 0.0;
        Self {
            objects,
            camera_pos,
            camera_rot,
        }
    }

    fn input(&mut self, ctx: &Context, dt: f32) {
        let rot_mat = Mat3::from_rotation_y(self.camera_rot);
        let rot_mat = rot_mat.to_cols_array_2d();
        let up = vec3(rot_mat[1][0], rot_mat[1][1], rot_mat[1][2]);
        let forward = vec3(rot_mat[2][0], rot_mat[2][1], rot_mat[2][2]).normalize();
        let right = vec3(rot_mat[0][0], rot_mat[0][1], rot_mat[0][2]);

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
            let path = "outputs/05.png";
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
            let distance = self.closest_sdf(pos);
            if distance < SURFACE_DISTANCE {
                return self.hit(ray_dir, pos);
            }

            t += distance;
            if t >= MAX_DISTANCE {
                break;
            }
        }
        self.miss()
    }

    fn hit(&self, rd: Vec3, pos: Vec3) -> Vec3 {
        let info = self.hitinfo(pos);
        let object = &self.objects[info.object_index];
        let normal = self.normal(pos);
        object.material.color(rd, pos, normal)
    }

    fn miss(&self) -> Vec3 {
        Vec3::ZERO
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        let center = self.hitinfo(pos).distance;
        let x = self.hitinfo(pos - vec3(EPSILON, 0.0, 0.0)).distance;
        let y = self.hitinfo(pos - vec3(0.0, EPSILON, 0.0)).distance;
        let z = self.hitinfo(pos - vec3(0.0, 0.0, EPSILON)).distance;
        (vec3(x, y, z) - center) / EPSILON
    }

    fn closest_sdf(&self, pos: Vec3) -> f32 {
        let mut closest = MAX_DISTANCE;
        for object in self.objects.iter() {
            let dist = object.surface.sdf(pos);
            if dist < closest {
                closest = dist;
            }
        }
        closest
    }

    fn hitinfo(&self, pos: Vec3) -> HitInfo {
        let mut closest = HitInfo {
            distance: MAX_DISTANCE,
            object_index: 0,
        };
        for (i, object) in self.objects.iter().enumerate() {
            let dist = object.surface.sdf(pos);
            if dist < closest.distance {
                closest = HitInfo {
                    distance: dist,
                    object_index: i,
                };
            }
        }
        closest
    }
}

fn main() {
    let app = Raymarcher::new();
    pixel_renderer::app::run(app)
}

struct HitInfo {
    distance: f32,
    object_index: usize,
}

struct Object {
    surface: Box<dyn Surface>,
    material: Box<dyn Material>,
}

impl Object {
    fn new(surface: Box<dyn Surface>, material: Box<dyn Material>) -> Self {
        Object { surface, material }
    }
}

trait Material {
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3) -> Vec3;
}

/// Flat color not affected by light
struct Flat {
    color: Vec3,
}

impl Flat {
    fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Flat {
    fn color(&self, _ray: Vec3, _pos: Vec3, _normal: Vec3) -> Vec3 {
        self.color
    }
}

/// Material that outputs the normal as a color
struct Normal;

impl Material for Normal {
    fn color(&self, _ray: Vec3, _pos: Vec3, normal: Vec3) -> Vec3 {
        normal
    }
}

/// Represents a surface defined by a SDF
trait Surface {
    fn sdf(&self, pos: Vec3) -> f32;
}

// Surface representing a sphere defined by position and radius
struct Sphere {
    pos: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }
}

impl Surface for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        self.pos.distance(pos) - self.radius
    }
}

/// Surface representing a plane defined by a normal
/// height defines distance moved along normal
struct Plane {
    normal: Vec3,
    height: f32,
}

impl Plane {
    fn new(normal: Vec3, height: f32) -> Self {
        let normal = normal.normalize();
        Self { normal, height }
    }
}

impl Surface for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.dot(self.normal) - self.height
    }
}
