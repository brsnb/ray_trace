use std::f64::consts::PI;

use crate::ray::Ray;
use crate::util::Vec3f;

pub struct Camera {
    origin: Vec3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
}

impl Camera {
    pub fn new(look_from: &Vec3f, look_at: &Vec3f, vup: &Vec3f, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;
        let origin = *look_from;

        let w = (*look_from - *look_at).as_unit();
        let u = vup.cross(&w).as_unit();
        let v = w.cross(&u);

        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
        )
    }
}
