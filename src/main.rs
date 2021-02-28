use ray_tracing::collision::*;
use ray_tracing::collision::{materials::*, objects::*};
use ray_tracing::*;

use rand::Rng;

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    rc::Rc,
};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMG_WIDTH: u32 = 800;
pub const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const MAX_DEPTH: i32 = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rand = rand::thread_rng();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("image.ppm")?;
    let mut file = BufWriter::new(file);

    // World
    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });

    let material_left = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });

    let material_center = Rc::new(Dielectric { ri: 1.3 });

    let material_right = Rc::new(Metal::new(Color::new(0.05, 0.05, 0.05), 0.0));

    let world = vec![
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground,
        },
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center.clone(),
        },
        /*
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: -0.4,
            material: material_center,
        },
        */
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left.clone(),
        },
        Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right,
        },
    ];

    let camera = Camera::default();

    // Render

    writeln!(&mut file, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT)?;

    let mut stderr = std::io::stderr();
    for j in (0..IMG_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr.flush()?;

        let j = j as f64;
        for i in 0..IMG_WIDTH {
            let i = i as f64;

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i + rand.gen::<f64>()) / (IMG_WIDTH as f64 - 1.0);
                let v = (j + rand.gen::<f64>()) / (IMG_HEIGHT as f64 - 1.0);

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world[..], &mut rand, MAX_DEPTH);
            }

            write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }

    eprintln!("\nDone.");
    Ok(())
}

fn ray_color<T: Hittable + ?Sized, R: Rng + ?Sized>(
    ray: &Ray,
    world: &T,
    rng: &mut R,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let material = hit.material.clone();
        if let Some((attenuation, scattered)) = material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, rng, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
