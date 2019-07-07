use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::util::{random_in_unit_sphere, Color, Vec3f};

use rand::prelude::*;

pub type Albedo = Color;
pub type Fuzz = f64;
pub type RefractiveIndex = f64;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    NoMaterial,
    Diffuse(Albedo),
    Metal(Albedo, Fuzz),
    Dielectric(RefractiveIndex),
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
            Material::Metal(a, f) => self.metal(ray_in, record, attenuation, scattered, &a, &f),
            Material::Dielectric(ri) => {
                self.dielectric(ray_in, record, attenuation, scattered, &ri)
            }
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
        fuzz: &Fuzz,
    ) -> bool {
        let fuzz: f64 = if *fuzz < 1.0 { *fuzz } else { 1.0 };
        let reflected = reflect(&ray_in.direction().as_unit(), &record.normal);
        *scattered = Ray::new(&record.p, &(reflected + random_in_unit_sphere() * fuzz));
        *attenuation = albedo.clone();
        scattered.direction().dot(&record.normal) > 0.0
    }

    fn dielectric(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        refractive_index: &RefractiveIndex,
    ) -> bool {
        let reflected = reflect(&ray_in.direction(), &record.normal);
        let mut outward_normal = Vec3f::new_unit_vec3f();
        let mut ni_over_nt: f64 = 0.0;
        let mut refracted = Vec3f::new_unit_vec3f();
        let mut cosine: f64 = 0.0;
        *attenuation = Color::new(1.0, 1.0, 1.0);

        if ray_in.direction().dot(&record.normal) > 0.0 {
            outward_normal = record.normal * -1.0;
            ni_over_nt = *refractive_index;
            cosine = *refractive_index * ray_in.direction().dot(&record.normal)
                / ray_in.direction().length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / *refractive_index;
            cosine = -(ray_in.direction().dot(&record.normal)) / ray_in.direction().length();
        }

        let mut reflect_prob: f64 = 0.0;

        if refract(
            &ray_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cosine, refractive_index);
        } else {
            reflect_prob = 1.0;
        }

        let r: f64 = random();

        if r < reflect_prob {
            *scattered = Ray::new(&record.p, &reflected);
        } else {
            *scattered = Ray::new(&record.p, &refracted);
        }

        true
    }
}

fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
    *v - (*n * v.dot(n) * 2.0)
}

fn refract(v: &Vec3f, n: &Vec3f, ni_over_nt: f64, refracted: &mut Vec3f) -> bool {
    let uv = v.as_unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = (uv - *n * dt) * ni_over_nt - *n * f64::sqrt(discriminant);
        return true;
    } else {
        return false;
    }
}

fn schlick(cosine: f64, refractive_index: &RefractiveIndex) -> f64 {
    let mut r0 = (1.0 - *refractive_index) / (1.0 + *refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

