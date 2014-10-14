use std::num::FloatMath;

use geometry::normal::Normal;
use geometry::matrix::Matrix;

#[deriving(Show)]
pub struct Vector {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Vector {
    pub fn new(x : f64, y : f64, z : f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector {
        Vector::new(0f64, 0f64, 0f64)
    }

    pub fn unit_x() -> Vector {
        Vector::new(1f64, 0f64, 0f64)
    }

    pub fn unit_y() -> Vector {
        Vector::new(0f64, 1f64, 0f64)
    }

    pub fn unit_z() -> Vector {
        Vector::new(0f64, 0f64, 1f64)
    }

    pub fn dot(&self, o : &Vector) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o : &Vector) -> Vector {
        Vector::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        if m == 0f64 { *self } else { self.div_s(m) }
    }

    pub fn normalize_self(&mut self) {
        let m = self.magnitude();
        if m != 0f64 { self.div_self_s(m) }
    }

    pub fn angle_between(&self, o : &Vector) -> f64 {
        (self.dot(o) / (self.magnitude() * o.magnitude())).acos()
    }

    pub fn to_normal(&self) -> Normal {
        Normal::new(self.x, self.y, self.z)
    }

    pub fn mul_m(&self, m : &Matrix) -> Vector {
        Vector::new(m[ 0] * self.x + m[ 4] * self.y + m[ 8] * self.z,
                    m[ 1] * self.x + m[ 5] * self.y + m[ 9] * self.z,
                    m[ 2] * self.x + m[ 6] * self.y + m[10] * self.z)
    }

    pub fn mul_self_m(&mut self, m : &Matrix) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        self.x = m[ 0] * x + m[ 4] * y + m[ 8] * z;
        self.y = m[ 1] * x + m[ 5] * y + m[ 9] * z;
        self.z = m[ 2] * x + m[ 6] * y + m[10] * z
    }

    pub fn mul_s(&self, s : f64) -> Vector {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mul_self_s(&mut self, s : f64) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s
    }

    pub fn div_s(&self, s : f64) -> Vector {
        Vector::new(self.x / s, self.y / s, self.z / s)
    }

    pub fn div_self_s(&mut self, s : f64) {
        self.x = self.x / s;
        self.y = self.y / s;
        self.z = self.z / s
    }

    pub fn add_v(&self, o : &Vector) -> Vector {
        Vector::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn add_self_v(&mut self, o : &Vector) {
        self.x = self.x + o.x;
        self.y = self.y + o.y;
        self.z = self.z + o.z
    }

    pub fn sub_v(&self, o : &Vector) -> Vector {
        Vector::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn sub_self_v(&mut self, o : &Vector) {
        self.x = self.x - o.x;
        self.y = self.y - o.y;
        self.z = self.z - o.z
    }

    pub fn reverse(&self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }

    pub fn reverse_self(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }

    fn clone_from(&mut self, source: &Vector) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Vector) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}
