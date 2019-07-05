use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::Vec3f;

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                record.material = self.material.clone();
                return true
            }
            temp = (-b + f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                record.material = self.material.clone();
                return true
            }
        }

        false
    }
}
