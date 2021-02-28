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

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w);
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random = self.lens_radius * Vec3::random_in_unit_disk(&mut rand::thread_rng());
        let on_plane = self.u * random.x() + self.v * random.y();

        Ray {
            origin: self.origin + on_plane,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - on_plane,
        }
    }
}
