use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            list: Vec::new(),
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;

        for i in 0..self.list.len() {
            if self.list[i].hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *record = temp_record.clone();
            }
        }
        hit_anything
    }
}
