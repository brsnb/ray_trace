use crate::hitable::{Hitable, HitRecord};
use crate::util::Vec3f;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;

        for i in 0..self.list.len() {
            if self.list[i].hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t.clone();
                *record = temp_record;
            }
        }
        hit_anything
    }
}

