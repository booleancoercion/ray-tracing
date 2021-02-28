use std::rc::Rc;

use super::{Hit, Hittable, Material, Ray};
use crate::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc: Vec3 = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = {
            let mut root = (-half_b - sqrtd) / a;
            if !(t_min..t_max).contains(&root) {
                root = (-half_b + sqrtd) / a;
                if !(t_min..t_max).contains(&root) {
                    return None;
                }
            }
            root
        };

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(Hit::with_face_normal(
            ray,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
