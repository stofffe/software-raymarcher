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

const SURFACE_DISTANCE: f32 = 0.0001;
const MAX_DISTANCE: f32 = 10.0;
const MAX_STEPS: u32 = 100;

/// Holds state needed for ray marcher
struct Raymarcher {
    spheres: Vec<Sphere>,
    camera_pos: Vec3,
}

impl Raymarcher {
    fn new() -> Self {
        let spheres = vec![
            Sphere::new(vec3(0.0, 0.0, 0.0), 1.0),
            Sphere::new(vec3(1.0, 1.0, -2.0), 1.0),
            Sphere::new(vec3(-1.0, -1.0, -0.5), 1.0),
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
            let path = "outputs/02.png";
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
                let result = self.raymarch(self.camera_pos, dir);
                if let Some(d) = result {
                    let scaled = d / MAX_DISTANCE;
                    let color = &[scaled, scaled, scaled];
                    canvas.write_pixel_f32(x, y, color);
                }
            }
        }
    }

    fn raymarch(&self, ro: Vec3, rd: Vec3) -> Option<f32> {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let pos = ro + rd * t;
            let dist = self.closest_sdf(pos);
            t += dist;
            if t > MAX_DISTANCE {
                break;
            }
            if dist < SURFACE_DISTANCE {
                return Some(t);
            }
        }
        None
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
}

fn main() {
    let app = Raymarcher::new();
    pixel_renderer::app::run(app)
}

// Distance functions
struct Sphere {
    pos: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }

    fn sdf(&self, pos: Vec3) -> f32 {
        self.pos.distance(pos) - self.radius
    }
}
