use glam::{vec3, Vec3};
use pixel_renderer::{
    app::{Callbacks, Config},
    canvas::Canvas,
    context::Context,
    input::KeyCode,
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
    spheres: Vec<Sphere>,
    camera_pos: Vec3,
}

impl Raymarcher {
    fn new() -> Self {
        let spheres = vec![
            Sphere::new(vec3(0.0, 0.0, 0.0), 1.0, RED),
            Sphere::new(vec3(1.0, 1.0, -2.0), 1.0, GREEN),
            Sphere::new(vec3(-1.0, -1.0, -0.5), 1.0, BLUE),
        ];
        let camera_pos = Vec3::new(0.0, 0.0, -5.0);
        Self {
            spheres,
            camera_pos,
        }
    }
}

impl Callbacks for Raymarcher {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        let canvas = &mut ctx.render.canvas;

        canvas.clear_screen();

        self.draw(canvas);

        if ctx.input.keyboard.key_just_pressed(KeyCode::S) {
            let path = "outputs/04.png";
            canvas.export_screenshot(path).unwrap();
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
    fn draw(&self, canvas: &mut Canvas) {
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                // Get uv coordinates and direction
                let uv = vec3(
                    x as f32 - WIDTH as f32 / 2.0,
                    y as f32 - HEIGHT as f32 / 2.0,
                    FOCAL_LENGTH,
                );
                let dir = uv.normalize();
                let color = self.raymarch(self.camera_pos, dir);
                canvas.write_pixel_f32(x, y, &color.to_array());
            }
        }
    }

    fn raymarch(&self, ro: Vec3, rd: Vec3) -> Vec3 {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let pos = ro + rd * t;
            let dist = self.closest_sdf(pos);
            if dist < SURFACE_DISTANCE {
                let color = self.closest_color(pos);
                let normal = self.normal(pos);
                return normal;
            }

            t += dist;
            if t > MAX_DISTANCE {
                break;
            }
        }
        Vec3::ZERO
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        let center = self.closest_sdf(pos);
        let x = self.closest_sdf(pos - vec3(EPSILON, 0.0, 0.0));
        let y = self.closest_sdf(pos - vec3(0.0, EPSILON, 0.0));
        let z = self.closest_sdf(pos - vec3(0.0, 0.0, EPSILON));
        (vec3(x, y, z) - center) / EPSILON
    }

    fn closest_sdf(&self, pos: Vec3) -> f32 {
        let mut closest = std::f32::MAX; // TODO option
        for sphere in self.spheres.iter() {
            let dist = sphere.sdf(pos);
            if dist < closest {
                closest = dist;
            }
        }
        closest
    }

    fn closest_color(&self, pos: Vec3) -> Vec3 {
        let mut closest_sphere = &self.spheres[0];
        let mut closest_dist = std::f32::MAX;
        for sphere in self.spheres.iter() {
            let dist = sphere.sdf(pos);
            if dist < closest_dist {
                closest_sphere = sphere;
                closest_dist = dist;
            }
        }
        closest_sphere.color
    }
}

fn main() {
    let app = Raymarcher::new();
    pixel_renderer::app::run(app)
}

// Distance functions
struct Sphere {
    pos: Vec3,
    radius: f32,
    color: Vec3,
}

impl Sphere {
    fn new(pos: Vec3, radius: f32, color: Vec3) -> Self {
        Self { pos, radius, color }
    }

    fn sdf(&self, pos: Vec3) -> f32 {
        self.pos.distance(pos) - self.radius
    }
}

