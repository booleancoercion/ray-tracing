pub mod collision;
mod vec3;

use std::io::Write;

pub use collision::Ray;
pub use vec3::Vec3;
pub type Color = Vec3;
pub type Point3 = Vec3;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub fn write_color<W: Write>(
    writer: &mut W,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    // divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    #[inline(always)]
    fn intify(x: f64) -> u32 {
        (256.0 * x.clamp(0.0, 0.999)) as u32
    }

    writeln!(writer, "{} {} {}", intify(r), intify(g), intify(b))
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 2.0;

        let origin = Point3::new(0.0, 0.0, 2.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
