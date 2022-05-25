use super::{Hit, Hittable, Material, Ray};
use crate::{Point3, Vec3};

use nalgebra::{Matrix3, Vector3};

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

        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
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

#[derive(Clone)]
pub struct Torus {
    pub center: Point3,
    pub major: f64, // commonly R
    pub minor: f64, // commonly r
    pub material: Arc<dyn Material>,
}

impl Hittable for Torus {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;

        let alpha = ray.direction.length_squared();
        let beta = 2.0 * oc.dot(&ray.direction);
        let gamma = oc.length_squared() + self.major * self.major - self.minor * self.minor;

        let mut o2 = oc;
        o2.0[2] = 0.0;

        let mut d2 = ray.direction;
        d2.0[2] = 0.0;

        #[allow(non_snake_case)]
        let R2_4 = 4.0 * self.major * self.major;

        let a4 = alpha * alpha;
        let a3 = 2.0 * alpha * beta;
        let a2 = 2.0 * alpha * gamma + beta * beta - R2_4 * d2.length_squared();
        let a1 = 2.0 * (beta * gamma - R2_4 * o2.dot(&d2));
        let a0 = gamma * gamma - R2_4 * o2.length_squared();

        let mut companion = nalgebra::Matrix5::from_fn(|i, j| (i == j + 1) as u8 as f64);
        companion.set_column(4, &[-a0, -a1, -a2, -a3, -a4].into());

        let solution = companion
            .complex_eigenvalues()
            .iter()
            .copied()
            .filter_map(|eigen| {
                if eigen.im.abs() > 1e-15 {
                    return None;
                }
                let re = eigen.re;
                if !(t_min..t_max).contains(&re) {
                    return None;
                }
                Some(re)
            })
            .fold(f64::INFINITY, |a, b| a.min(b));

        if solution.is_infinite() {
            return None;
        }

        let pt = ray.at(solution) - self.center;
        let (x, y) = (pt.x(), pt.y());
        let coeff = self.major * self.major / (x * x + y * y).sqrt();

        let normal = Vec3::new(1.0 - coeff * x, 1.0 - coeff * y, pt.z()).normalize();

        Some(Hit::with_face_normal(
            ray,
            normal,
            solution,
            self.material.clone(),
        ))
    }
}

const TRIPLETS: [(usize, usize, usize); 3] = [(1, 2, 0), (2, 0, 1), (0, 1, 2)];

#[derive(Clone)]
pub struct Parallelogram {
    corner: Vector3<f64>,
    axes: [Vector3<f64>; 3],
    normals: [Vector3<f64>; 3],
    pub material: Arc<dyn Material>,
}

impl Parallelogram {
    pub fn new(corner: Point3, u: Vec3, v: Vec3, w: Vec3, material: Arc<dyn Material>) -> Self {
        let corner: Vector3<f64> = corner.into();

        let u: Vector3<f64> = u.into();
        let v: Vector3<f64> = v.into();
        let w: Vector3<f64> = w.into();

        fn cross_with_dir(
            x: &Vector3<f64>,
            y: &Vector3<f64>,
            direction: &Vector3<f64>,
        ) -> Vector3<f64> {
            let v: Vector3<f64> = x.cross(y);
            if v.dot(direction) < 0.0 {
                -v
            } else {
                v
            }
        }

        let vw: Vector3<f64> = cross_with_dir(&v, &w, &u).normalize();
        let uw: Vector3<f64> = cross_with_dir(&u, &w, &v).normalize();
        let uv: Vector3<f64> = cross_with_dir(&u, &v, &w).normalize();

        Self {
            corner,
            axes: [u, v, w],
            normals: [vw, uw, uv],
            material,
        }
    }
}

impl Hittable for Parallelogram {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut t = t_max;
        let mut normal: Option<Vector3<f64>> = None;

        let ro: Vector3<f64> = ray.origin.into();
        let rd: Vector3<f64> = ray.direction.into();

        let mut lu;
        let mut floor: Vector3<f64>;
        let mut ceiling: Vector3<f64>;
        // in each iteration, go for the two parallel planes generated by the x and y vectors
        for (x, y, z) in TRIPLETS.iter().copied() {
            lu = Matrix3::from_columns(&[self.axes[x], self.axes[y], -rd]).lu();
            floor = ro - self.corner;
            ceiling = ro - self.corner - self.axes[z];

            let floor_solved = lu.solve_mut(&mut floor);
            let ceiling_solved = lu.solve_mut(&mut ceiling);

            if floor_solved
                && (t_min..t).contains(&floor[2])
                && (0.0..1.0).contains(&floor[0])
                && (0.0..1.0).contains(&floor[1])
            {
                t = floor[2];
                normal = Some(-self.normals[z]);
            }

            if ceiling_solved
                && (t_min..t).contains(&ceiling[2])
                && (0.0..1.0).contains(&ceiling[0])
                && (0.0..1.0).contains(&ceiling[1])
            {
                t = ceiling[2];
                normal = Some(self.normals[z]);
            }
        }

        let normal = normal?.into();

        Some(Hit::with_face_normal(ray, normal, t, self.material.clone()))
    }
}
