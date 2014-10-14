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
        self.x*o.x + self.y*o.y + self.z*o.z
    }

    pub fn cross(&self, o : &Vector) -> Vector {
        Vector::new(self.y*o.z-self.z*o.y, self.z*o.x-self.x*o.z, self.x*o.y-self.y*o.x)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        if m == 0f64 { *self } else { self / m }
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
}

impl Mul<f64, Vector> for Vector {
    fn mul(&self, s : &f64) -> Vector {
        Vector::new(self.x * *s, self.y * *s, self.z * *s)
    }
}

impl Div<f64, Vector> for Vector {
    fn div(&self, s : &f64) -> Vector {
        Vector::new(self.x / *s, self.y / *s, self.z / *s)
    }
}

impl Add<Vector, Vector> for Vector {
    fn add(&self, o : &Vector) -> Vector {
        Vector::new(self.x+o.x, self.y+o.y, self.z+o.z)
    }
}

impl Sub<Vector, Vector> for Vector {
    fn sub(&self, o : &Vector) -> Vector {
        Vector::new(self.x-o.x, self.y-o.y, self.z-o.z)
    }
}

impl Neg<Vector> for Vector {
    fn neg(&self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}
