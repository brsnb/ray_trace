use crate::ray::Ray;
use crate::util::Vec3f;

pub struct Camera {
    origin: Vec3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Vec3f::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3f::new(4.0, 0.0, 0.0);
        let vertical = Vec3f::new(0.0, 2.0, 0.0);
        let origin = Vec3f::new(0.0, 0.0, 0.0);

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
