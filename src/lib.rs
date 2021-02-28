pub mod collision;
mod vec3;

pub use collision::Ray;
pub use vec3::Vec3;
pub type Color = Vec3;
pub type Point3 = Vec3;

pub fn color_to_rgb(pixel_color: Color, samples_per_pixel: u32) -> image::Rgb<u8> {
    // divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    #[inline(always)]
    fn intify(x: f64) -> u8 {
        (256.0 * x.clamp(0.0, 0.999)) as u8
    }

    image::Rgb([intify(r), intify(g), intify(b)])
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lower_left_corner: Point3,
        horizontal: Vec3,
        vertical: Vec3,
    ) -> Self {
        assert!(!horizontal.near_zero());
        assert!(!vertical.near_zero());

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
