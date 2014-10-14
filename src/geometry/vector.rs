use std::num::FloatMath;

use geometry::normal::Normal;
use geometry::transform::{Transform,Transformable};

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

impl Transformable for Vector {
    fn transform(&self, t : &Transform) -> Vector {
        t.transformation_matrix().mul_v(self)
    }

    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.transformation_matrix().mul_v(&c))
    }
}

#[test]
fn test_accessors() {
    assert_eq!(Vector::new(1f64, 2f64, 3f64).x, 1f64);
    assert_eq!(Vector::new(1f64, 2f64, 3f64).y, 2f64);
    assert_eq!(Vector::new(1f64, 2f64, 3f64).z, 3f64);
}

#[test]
fn test_equality() {
    assert!(Vector::zero() == Vector::zero());
    assert!(Vector::zero() == Vector::new(0f64, 0f64, 0f64));
    assert!(Vector::zero() != Vector::new(1f64, 0f64, 0f64));
    assert!(Vector::zero() != Vector::new(0f64, 1f64, 0f64));
    assert!(Vector::zero() != Vector::new(0f64, 0f64, 1f64));
    assert!(Vector::unit_x() == Vector::unit_x());
    assert!(Vector::unit_x() != Vector::unit_y());
}

#[test]
fn test_dot() {
    assert_eq!(Vector::new(1f64, 2f64, 3f64).dot(&Vector::zero()), 0f64);
    assert_eq!(Vector::new(1f64, 2f64, 3f64).dot(&Vector::unit_y()), 2f64);
    assert_eq!(Vector::new(1f64, 2f64, 3f64).dot(&Vector::new(4f64, 5f64, 6f64)), 32f64);
}

#[test]
fn test_cross() {
    assert_eq!(Vector::unit_x().cross(&Vector::unit_y()), Vector::unit_z());
}

#[test]
fn test_magnitude() {
    assert_eq!(Vector::zero().magnitude(), 0f64);
    assert_eq!(Vector::unit_x().magnitude(), 1f64);
}

#[test]
fn test_normalize() {
    assert_eq!(Vector::unit_x().mul_s(3f64).normalize(), Vector::unit_x());
}

#[test]
fn test_reverse() {
    assert_eq!(Vector::zero().reverse(), Vector::zero());
    assert_eq!(Vector::new(1f64, -2f64, 3f64).reverse(), Vector::new(-1f64, 2f64, -3f64));
}

#[test]
fn test_add() {
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_x()), Vector::new(2f64, 0f64, 0f64));
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_y()), Vector::new(1f64, 1f64, 0f64));
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_z()), Vector::new(1f64, 0f64, 1f64));

    let mut v = Vector::unit_x();
    v.add_self_v(&Vector::unit_x());
    v.add_self_v(&Vector::unit_y());
    assert_eq!(v, Vector::new(2f64, 1f64, 0f64));
}

#[test]
fn test_mul() {
    assert_eq!(Vector::unit_x().mul_s(3f64), Vector::new(3f64, 0f64, 0f64));
    assert_eq!(Vector::unit_y().mul_s(3f64), Vector::new(0f64, 3f64, 0f64));

    let mut v = Vector::unit_x();
    v.mul_self_s(3f64);
    assert_eq!(v, Vector::new(3f64, 0f64, 0f64));
}

#[test]
fn test_div() {
    assert_eq!(Vector::unit_x().mul_s(3f64).div_s(3f64), Vector::unit_x());
    assert_eq!(Vector::unit_y().mul_s(3f64).div_s(3f64), Vector::unit_y());

    let mut v = Vector::unit_x();
    v.mul_self_s(3f64);
    v.div_self_s(3f64);
    assert_eq!(v, Vector::unit_x());
}

#[test]
fn test_angle() {
    assert_eq!(Vector::unit_x().angle_between(&Vector::unit_y()), Float::frac_pi_2());
    assert_eq!(Vector::unit_y().angle_between(&Vector::unit_x()), Float::frac_pi_2());
}
