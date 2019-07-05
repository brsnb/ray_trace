use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::util::{Color, Vec3f};

use rand::prelude::*;

pub type Albedo = Color;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    NoMaterial,
    Diffuse(Albedo),
    Metal(Albedo),
}

impl Material {
    pub fn new() -> Material {
        Material::NoMaterial
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::NoMaterial => true,
            Material::Diffuse(a) => self.diffuse(ray_in, record, attenuation, scattered, &a),
            Material::Metal(a) => self.metal(ray_in, record, attenuation, scattered, &a),
        }
    }

    fn diffuse(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        albedo: &Color,
    ) -> bool {
        let target = record.p + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(&record.p, &(target - record.p));
        *attenuation = albedo.clone();
        true
    }

    fn metal(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        albedo: &Color,
    ) -> bool {
        let reflected = Material::reflect(&ray_in.direction().as_unit(), &record.normal);
        *scattered = Ray::new(&record.p, &reflected);
        *attenuation = albedo.clone();
        scattered.direction().dot(&record.normal) > 0.0
    }

    fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
        *v - (*n * v.dot(n) * 2.0)
    }
}

pub fn random_in_unit_sphere() -> Vec3f {
    let mut rng = thread_rng();
    loop {
        let p = (Vec3f::new(rng.gen(), rng.gen(), rng.gen()) - Vec3f::new(1.0, 1.0, 1.0)) * 2.0;
        if p.squared_length() >= 1.0 {
            break p;
        }
    }
}

