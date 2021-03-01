use super::{Hit, Hittable, Material, Ray};
use crate::{Point3, Vec3};

use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc: Vec3 = ray.origin - self.center;

        let (a, half_b, c) = {
            let [dx, dy, dz] = ray.direction.0;
            let [ocx, ocy, ocz] = oc.0;
            let r = self.radius;

            (
                dx * dx + dy * dy + dz * dz,
                dx * ocx + dy * ocy + dz * ocz,
                ocx * ocx + ocy * ocy + ocz * ocz - r * r,
            )
        };
        /*
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        */

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = {
            let nhalf_b_a = -half_b / a;
            let sqrtd_a = sqrtd / a;

            let mut root = nhalf_b_a - sqrtd_a;
            if root < t_min || t_max < root {
                root = nhalf_b_a + sqrtd_a;
                if root < t_min || t_max < root {
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
