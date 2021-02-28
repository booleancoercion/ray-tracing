use crate::{Color, Point3, Vec3};

use std::sync::Arc;

pub mod materials;
pub mod objects;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn hit<T: Hittable>(&self, hittable: &T, t_min: f64, t_max: f64) -> Option<Hit> {
        hittable.hit(self, t_min, t_max)
    }
}

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Hit {
    pub fn with_face_normal(
        ray: &Ray,
        outward_normal: Vec3,
        t: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point: ray.at(t),
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<T: Hittable> Hittable for [T] {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<Hit> {
        let mut closest_hit = None;

        for object in self {
            if let Some(hit) = object.hit(ray, t_min, t_max) {
                t_max = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        (&self[..]).hit(ray, t_min, t_max)
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}
