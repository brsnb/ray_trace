use std::f64;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

//TODO: Implement iter maybe
#[derive(Copy, Clone, Debug)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn new_unit_vec3f() -> Vec3f {
        Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn into_unit(&mut self) {
        let k = 1.0 / self.length();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn as_unit(&self) -> Vec3f {
        let len = self.length();

        Vec3f {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn squared_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<f64> for Vec3f {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul<f64> for Vec3f {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Color> for Vec3f {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        Self {
            x: self.x * other.r,
            y: self.y * other.g,
            z: self.z * other.b,
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub b: f64,
    pub g: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}


impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

