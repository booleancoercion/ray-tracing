use ray_tracing::collision::*;
use ray_tracing::collision::{materials::*, objects::*};
use ray_tracing::*;

use image::ImageBuffer;
use image::Rgb as GenericRgb;
use rand::Rng;

use std::io::Write;
use std::mem::MaybeUninit;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMG_WIDTH: u32 = 1200;
pub const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 10;
pub const MAX_DEPTH: i32 = 500;

type Rgb = GenericRgb<u8>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threads = num_cpus::get() as u32;

    // World

    let world = {
        let mut rand = rand::thread_rng();
        let mut world = Vec::new();

        let ground_material = Arc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        world.push(Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: ground_material,
        });

        for a in (-11)..11 {
            let a = a as f64;
            for b in (-11)..11 {
                let b = b as f64;

                let center = Point3::new(
                    a + 0.9 * rand.gen::<f64>(),
                    0.2,
                    b + 0.9 * rand.gen::<f64>(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let choose_mat: f64 = rand.gen();
                    let material: Arc<dyn Material> = if choose_mat < 0.8 {
                        // diffuse
                        let albedo = rand.gen::<Color>() * rand.gen::<Color>();
                        Arc::new(Lambertian { albedo })
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = rand.gen::<Color>() * 0.5 + Color::new(0.5, 0.5, 0.5);
                        let fuzz: f64 = rand.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        // glass
                        Arc::new(Dielectric { ri: 1.5 })
                    };

                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material,
                    })
                }
            }
        }

        let material1 = Arc::new(Dielectric { ri: 1.5 });
        let material2 = Arc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

        world.push(Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: material1,
        });
        world.push(Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: material2,
        });
        world.push(Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: material3,
        });

        Arc::new(world)
    };

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    ));

    // Render

    let (tx, rx) = mpsc::channel();

    for tid in 0..threads {
        generate_thread(tid, threads, tx.clone(), camera.clone(), world.clone());
    }

    drop(tx); // the extra sender would cause a deadlock in the current thread

    let mut buf = ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);
    let mut stderr = std::io::stderr();
    let mut count = IMG_HEIGHT;
    while let Ok((posy, row)) = rx.recv() {
        let posy = IMG_HEIGHT - posy - 1;
        count -= 1;
        eprint!("\rScanlines remaining: {} ", count);
        stderr.flush()?;
        for (posx, &pixel) in row.iter().enumerate() {
            buf.put_pixel(posx as u32, posy, pixel);
        }
    }

    buf.save("output.png")?;

    eprintln!("\nDone.");
    Ok(())
}

fn generate_thread<T>(
    id: u32,
    threads: u32,
    sender: Sender<(u32, [Rgb; IMG_WIDTH as usize])>,
    camera: Arc<Camera>,
    world: Arc<T>,
) -> JoinHandle<()>
where
    T: Hittable + ?Sized + Send + Sync + 'static,
{
    const SIZE: usize = IMG_WIDTH as usize;

    thread::spawn(move || {
        let mut rand = rand::thread_rng();

        for jdx in (0..IMG_HEIGHT).filter(|x| x.rem_euclid(threads) == id) {
            let j = jdx as f64;

            let mut row: [MaybeUninit<Rgb>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
            for (idx, elem) in row.iter_mut().enumerate() {
                let i = idx as f64;

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i + rand.gen::<f64>()) / (IMG_WIDTH as f64 - 1.0);
                    let v = (j + rand.gen::<f64>()) / (IMG_HEIGHT as f64 - 1.0);

                    let ray = camera.get_ray(u, v);

                    pixel_color += ray_color(&ray, &world, &mut rand, MAX_DEPTH);
                }

                let rgb = color_to_rgb(pixel_color, SAMPLES_PER_PIXEL);
                *elem = MaybeUninit::new(rgb);
            }

            let row = unsafe { std::mem::transmute::<_, [Rgb; SIZE]>(row) };

            sender.send((jdx, row)).unwrap();
        }
    })
}

fn ray_color<T: Hittable + ?Sized, R: Rng + ?Sized>(
    ray: &Ray,
    world: &Arc<T>,
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
