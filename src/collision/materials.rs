use super::{Hit, Material};
use crate::{Color, Ray, Vec3};

use std::rc::Rc;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(self: Rc<Self>, _: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let dir: Vec3 = hit.normal + Vec3::random_unit_vec(&mut rand::thread_rng());

            // Catch degenerate scatter direction
            if dir.near_zero() {
                hit.normal
            } else {
                dir
            }
        };

        Some((
            self.albedo,
            Ray {
                origin: hit.point,
                direction: scatter_direction,
            },
        ))
    }
}

pub struct Metal {
    pub albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz.abs() < 1.0 { fuzz } else { 1.0 };

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(self: Rc<Self>, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray.direction, &hit.normal); // Maybe normalize direction??

        // Optimization in case there is no fuzz
        let direction = if self.fuzz == 0.0 {
            reflected
        } else {
            reflected + self.fuzz * Vec3::random_in_unit_sphere(&mut rand::thread_rng())
        };

        if reflected.dot(&hit.normal) > 0.0 {
            Some((
                self.albedo,
                Ray {
                    origin: hit.point,
                    direction,
                },
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    // refraction index
    pub ri: f64,
}

impl Material for Dielectric {
    fn scatter(self: Rc<Self>, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ri
        } else {
            self.ri
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(&unit_direction, &hit.normal)
            } else {
                refract(&unit_direction, &hit.normal, refraction_ratio)
            };

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray {
                origin: hit.point,
                direction,
            },
        ))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(r_in: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-*r_in).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (*r_in + cos_theta * *normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * *normal;

    r_out_perp + r_out_parallel
}
