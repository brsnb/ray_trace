use crate::util::Vec3f;

pub struct Ray {
    a: Vec3f,
    b: Vec3f,
}

impl Ray {
    pub fn new(a: &Vec3f, b: &Vec3f) -> Ray {
        Ray {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn origin(&self) -> Vec3f {
        self.a.clone()
    }

    pub fn direction(&self) -> Vec3f {
        self.b.clone()
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3f {
        self.a + self.b * t
    }
}
