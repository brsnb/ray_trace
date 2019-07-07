use std::f64::consts::PI;

use crate::ray::Ray;
use crate::util::{random_in_unit_sphere, Vec3f};

pub struct Camera {
    origin: Vec3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u: Vec3f,
    v: Vec3f,
    w: Vec3f,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: &Vec3f,
        look_at: &Vec3f,
        vup: &Vec3f,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;
        let origin = *look_from;

        let w = (*look_from - *look_at).as_unit();
        let u = vup.cross(&w).as_unit();
        let v = w.cross(&u);

        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2.0 * half_width * focus_dist;
        let vertical = v * 2.0 * half_height * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset),
        )
    }
}
