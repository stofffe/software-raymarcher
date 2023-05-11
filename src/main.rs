use glam::{vec3, vec4, Vec3, Vec4, Vec4Swizzles};
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

/// Holds state needed for ray marcher
struct Raymarcher {
    surfaces: Vec<Box<dyn Surface>>,
    camera_pos: Vec3,
}

impl Raymarcher {
    fn new() -> Self {
        let surfaces: Vec<Box<dyn Surface>> = vec![
            Box::new(Sphere::new(vec3(0.0, 0.0, 0.0), 1.0, RED)),
            Box::new(Sphere::new(vec3(1.0, 1.0, -2.0), 1.0, GREEN)),
            Box::new(Plane::new(vec3(0.0, -1.0, 0.0), -3.0, BLUE)),
        ];
        let camera_pos = Vec3::new(0.0, 0.0, -5.0);
        Self {
            surfaces,
            camera_pos,
        }
    }
}

impl Callbacks for Raymarcher {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        canvas::clear_screen(ctx);

        self.draw(ctx);

        if keyboard::key_just_pressed(ctx, KeyCode::S) {
            let path = "outputs/04.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }

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
    fn draw(&self, ctx: &mut Context) {
        for y in 0..canvas::height(ctx) {
            for x in 0..canvas::width(ctx) {
                // Get uv coordinates and direction
                let uv = vec3(
                    x as f32 - WIDTH as f32 / 2.0,
                    y as f32 - HEIGHT as f32 / 2.0,
                    FOCAL_LENGTH,
                );
                let dir = uv.normalize();
                let color = self.raymarch(self.camera_pos, dir);
                canvas::write_pixel_f32(ctx, x, y, &color.to_array());
            }
        }
    }

    fn raymarch(&self, ro: Vec3, rd: Vec3) -> Vec3 {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let pos = ro + rd * t;
            let closest = self.closest_sdf(pos);
            if closest.w < SURFACE_DISTANCE {
                let normal = self.normal(pos);
                let color = closest.xyz();
                return normal;
            }

            t += closest.w;
            if t > MAX_DISTANCE {
                break;
            }
        }
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
        let mut closest = vec4(0.0, 0.0, 0.0, std::f32::MAX);
        for surface in self.surfaces.iter() {
            let dist = surface.sdf(pos);
            if dist < closest.w {
                closest = vec4(
                    surface.color().x,
                    surface.color().y,
                    surface.color().z,
                    dist,
                )
            }
        }
        closest
    }
}

fn main() {
    let app = Raymarcher::new();
    pixel_renderer::app::run(app)
}

/// Represents a surface defined by a SDF
trait Surface {
    fn sdf(&self, pos: Vec3) -> f32;
    fn color(&self) -> Vec3;
}

// Surface representing a sphere defined by position and radius
struct Sphere {
    pos: Vec3,
    radius: f32,
    color: Vec3,
}

impl Sphere {
    fn new(pos: Vec3, radius: f32, color: Vec3) -> Self {
        Self { pos, radius, color }
    }
}

impl Surface for Sphere {
    fn sdf(&self, pos: Vec3) -> f32 {
        self.pos.distance(pos) - self.radius
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

/// Surface representing a plane defined by a normal
/// height defines distance moved along normal
struct Plane {
    normal: Vec3,
    height: f32,
    color: Vec3,
}

impl Plane {
    fn new(normal: Vec3, height: f32, color: Vec3) -> Self {
        let normal = normal.normalize();
        Self {
            normal,
            height,
            color,
        }
    }
}

impl Surface for Plane {
    fn sdf(&self, pos: Vec3) -> f32 {
        pos.dot(self.normal) - self.height
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}
