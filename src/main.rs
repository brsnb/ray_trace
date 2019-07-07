use std::f64;

use ray_trace::camera::Camera;
use ray_trace::hitable::{HitRecord, Hitable};
use ray_trace::hitable_list::HitableList;
use ray_trace::material::Material;
use ray_trace::ray::Ray;
use ray_trace::sphere::Sphere;
use ray_trace::util::{Color, Vec3f};

use rand::prelude::*;

pub fn color(ray: &Ray, world: &impl Hitable, depth: u32) -> Color {
    let mut record = HitRecord::new();
    if world.hit(ray, 0.001, f64::MAX, &mut record) {
        let mut scattered = Ray::new(&Vec3f::new_unit_vec3f(), &Vec3f::new_unit_vec3f());
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if depth < 50
            && record
                .material
                .scatter(ray, &record, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    } else {
        let t = (ray.direction().as_unit().y + 1.0) * 0.5;
        (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}

pub fn gen_ppm() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut world = HitableList::new();
    world.list.push(Box::new(Sphere::new(
        Vec3f::new(0.0, 0.0, -1.0),
        0.5,
        Material::Diffuse(Color::new(0.1, 0.2, 0.5)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3f::new(0.0, -100.5, -1.0),
        100.0,
        Material::Diffuse(Color::new(0.8, 0.8, 0.0)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3f::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3f::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(1.5),
    )));

    let camera = Camera::new(
        &Vec3f::new(-2.0, 2.0, 1.0),
        &Vec3f::new(0.0, 0.0, -1.0),
        &Vec3f::new(0.0, 1.0, 0.0),
        75.0,
        nx as f64 / ny as f64,
    );

    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            let mut rng = thread_rng();
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u = (u_rand + i as f64) / nx as f64;
                let v = (v_rand + j as f64) / ny as f64;
                let ray = camera.get_ray(u, v);
                col = col + color(&ray, &world, 0);
            }

            col = col / ns as f64;
            col = Color::new(f64::sqrt(col.r), f64::sqrt(col.g), f64::sqrt(col.b));

            let ir = (255.99 * col.r) as u32;
            let ig = (255.99 * col.g) as u32;
            let ib = (255.99 * col.b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    gen_ppm();
}
