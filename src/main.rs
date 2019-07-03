use std::f64;

use rand::prelude::*;

use ray_trace::camera::Camera;
use ray_trace::hitable::{HitRecord, Hitable};
use ray_trace::hitable_list::HitableList;
use ray_trace::ray::Ray;
use ray_trace::sphere::Sphere;
use ray_trace::util::{Color, Vec3f};

fn hit_sphere(center: &Vec3f, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - *center;
    let a = ray.direction().dot(&ray.direction());
    let b = oc.dot(&ray.direction()) * 2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        (-b - f64::sqrt(discriminant)) / (a * 2.0)
    }
}

fn color(ray: &Ray, world: &impl Hitable) -> Color {
    let mut record = HitRecord::new();
    if world.hit(ray, 0.0, f64::MAX, &mut record) {
        return Color::new(
            record.normal.x + 1.0,
            record.normal.y + 1.0,
            record.normal.z + 1.0,
        ) * 0.5;
    }

    let t = (ray.direction().as_unit().y + 1.0) * 0.5;
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn gen_ppm() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut world: HitableList = Default::default();
    world
        .list
        .push(Box::new(Sphere::new(Vec3f::new(0.0, 0.0, -1.0), 0.5)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3f::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            let mut rng = thread_rng();
            let mut col = Color::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u = (u_rand + i as f64) / nx as f64;
                let v = (v_rand + j as f64) / ny as f64;
                let ray = camera.get_ray(u, v);
                col = col + color(&ray, &world);
            }

            col = col / ns as f64;

            let ir = (255.99 * col.r) as i32;
            let ig = (255.99 * col.g) as i32;
            let ib = (255.99 * col.b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    gen_ppm();
}
