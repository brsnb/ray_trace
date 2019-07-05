use crate::material::Material;
use crate::ray::Ray;
use crate::util::Vec3f;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3f,
    pub normal: Vec3f,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3f::new_unit_vec3f(),
            normal: Vec3f::new_unit_vec3f(),
            material: Material::new(),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
