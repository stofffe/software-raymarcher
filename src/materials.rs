use glam::{vec3, Vec3};
use image::{DynamicImage, GenericImageView, Pixel};

pub const RED: Vec3 = vec3(1.0, 0.0, 0.0);
pub const GREEN: Vec3 = vec3(0.0, 1.0, 0.0);
pub const BLUE: Vec3 = vec3(0.0, 0.0, 1.0);
pub const WHITE: Vec3 = vec3(1.0, 1.0, 1.0);
pub const YELLOW: Vec3 = vec3(1.0, 1.0, 0.0);
pub const PINK: Vec3 = vec3(1.0, 0.5, 0.5);

pub trait MaterialTrait {
    fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3;
}

/// Material that outputs a flat color
pub struct Unlit {
    color: Vec3,
}

impl Unlit {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl MaterialTrait for Unlit {
    fn color(&self, _ray: Vec3, _pos: Vec3, _normal: Vec3, _light_pos: Vec3) -> Vec3 {
        self.color
    }
}

/// Material that outputs the normal as a color
pub struct Normal;

impl MaterialTrait for Normal {
    fn color(&self, _ray: Vec3, _pos: Vec3, normal: Vec3, _light_pos: Vec3) -> Vec3 {
        normal
    }
}

// Material that samples from a pixel based of world position
pub struct Textured {
    texture: Texture,
    blend_sharpness: f32,
    scale: f32,
}

impl Textured {
    pub fn new(texture_path: &str) -> Self {
        let texture = Texture::new(texture_path);
        Self {
            texture,
            blend_sharpness: 1.0,
            scale: 1.0,
        }
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_blend_sharpness(mut self, k: f32) -> Self {
        self.blend_sharpness = k;
        self
    }
}

impl MaterialTrait for Textured {
    fn color(&self, _ray: Vec3, pos: Vec3, normal: Vec3, _light_pos: Vec3) -> Vec3 {
        let x = self.texture.sample(pos.y * self.scale, pos.z * self.scale);
        let y = self.texture.sample(pos.z * self.scale, pos.x * self.scale);
        let z = self.texture.sample(pos.x * self.scale, pos.y * self.scale);

        // let mut weight = normal.abs();
        let mut weight = normal.abs().powf(self.blend_sharpness);
        weight = weight / (weight.x + weight.y + weight.z);

        x * weight.x + y * weight.y + z * weight.z
    }
}

/// Uses repeating if sampled outside unit quad
pub struct Texture {
    image: DynamicImage,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).unwrap();
        Self { image }
    }

    /// Returns the color of the pixel at world position (x,y)
    pub fn sample(&self, x: f32, y: f32) -> Vec3 {
        // Turn world position into texture position [0,1] range
        let mut x_scaled = x % 1.0;
        if x_scaled < 0.0 {
            x_scaled += 1.0;
        }
        let mut y_scaled = y % 1.0;
        if y_scaled < 0.0 {
            y_scaled += 1.0;
        }

        // Turn [0,1] range to texture position
        let x_screen = (x_scaled % 1.0) * self.image.width() as f32;
        let y_screen = (y_scaled % 1.0) * self.image.height() as f32;

        let rgba = self
            .image
            .get_pixel(x_screen as u32, y_screen as u32)
            .to_rgba();
        vec3(
            rgba[0] as f32 / 255.0,
            rgba[1] as f32 / 255.0,
            rgba[2] as f32 / 255.0,
        )
    }
}

// Material that outputs a shaded color
// Uses phong shading
// pub struct PhongShadedTexture {
//     texture: Textured,
// }
//
// impl PhongShadedTexture {
//     pub fn new(texture: Textured) -> Self {
//         Self { texture }
//     }
// }
//
// impl Material for PhongShadedTexture {
//     fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
//         let light_dir = pos - light_pos;
//         let light = Vec3::dot(normal.normalize(), light_dir.normalize()).max(0.0);
//         let color = self.texture.color(ray, pos, normal, light_pos);
//         color * (light + INDIRECT_LIGHT)
//     }
// }
// Material that outputs a shaded color
// Uses phong shading
// pub struct PhongShaded {
//     color: Vec3,
// }
//
// impl PhongShaded {
//     pub fn new(color: Vec3) -> Self {
//         Self { color }
//     }
// }
//
// impl Material for PhongShaded {
//     fn color(&self, _ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
//         let light_dir = pos - light_pos;
//         let light = Vec3::dot(normal.normalize(), light_dir.normalize()).max(0.0);
//         self.color * (light + INDIRECT_LIGHT)
//     }
// }

// Material that outputs a shaded color
// Uses phong shading
// pub struct Shaded {
//     texture: Box<dyn Material>,
// }
//
// impl Shaded {
//     pub fn new(texture: Box<dyn Material>) -> Self {
//         Self { texture }
//     }
// }
//
// impl Material for Shaded {
//     fn color(&self, ray: Vec3, pos: Vec3, normal: Vec3, light_pos: Vec3) -> Vec3 {
//         let light_dir = pos - light_pos;
//         let light = Vec3::dot(normal.normalize(), light_dir.normalize()).max(0.0);
//         let color = self.texture.color(ray, pos, normal, light_pos);
//         color * (light + INDIRECT_LIGHT)
//     }
// }
