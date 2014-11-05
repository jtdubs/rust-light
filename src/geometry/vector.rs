use std::num::FloatMath;
use std::fmt::{Show,Formatter,Result};

use geometry::normal::Normal;
use geometry::transform::{Transform,Trans,TransMut};

pub struct Vector {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector {
    pub fn new(x : f32, y : f32, z : f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector {
        Vector::new(0f32, 0f32, 0f32)
    }

    pub fn unit_x() -> Vector {
        Vector::new(1f32, 0f32, 0f32)
    }

    pub fn unit_y() -> Vector {
        Vector::new(0f32, 1f32, 0f32)
    }

    pub fn unit_z() -> Vector {
        Vector::new(0f32, 0f32, 1f32)
    }

    pub fn dot(&self, o : &Vector) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o : &Vector) -> Vector {
        Vector::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        if m == 0f32 { *self } else { self / m }
    }

    pub fn normalize_self(&mut self) {
        let m = self.magnitude();
        if m != 0f32 { self.div_self_s(m) }
    }

    pub fn angle_between(&self, o : &Vector) -> f32 {
        (self.dot(o) / (self.magnitude() * o.magnitude())).acos()
    }

    pub fn to_normal(&self) -> Normal {
        Normal::new(self.x, self.y, self.z)
    }

    pub fn mul_s(&self, s : f32) -> Vector {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mul_self_s(&mut self, s : f32) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s
    }

    pub fn div_s(&self, s : f32) -> Vector {
        Vector::new(self.x / s, self.y / s, self.z / s)
    }

    pub fn div_self_s(&mut self, s : f32) {
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

    pub fn mul_v(&self, v : &Vector) -> Vector {
        Vector::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

    pub fn mul_self_v(&mut self, v : &Vector) {
        self.x = self.x * v.x;
        self.y = self.y * v.y;
        self.z = self.z * v.z;
    }

    pub fn div_v(&self, v : &Vector) -> Vector {
        Vector::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }

    pub fn div_self_v(&mut self, v : &Vector) {
        self.x = self.x / v.x;
        self.y = self.y / v.y;
        self.z = self.z / v.z;
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

impl Show for Vector {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "<{}, {}, {}>", self.x, self.y, self.z)
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

impl Trans for Vector {
    fn transform(&self, t : &Transform) -> Vector {
        t.transformation_matrix().mul_v(self)
    }
}

impl TransMut for Vector {
    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.transformation_matrix().mul_v(&c))
    }
}

impl Add<Vector, Vector> for Vector {
    fn add(&self, v : &Vector) -> Vector {
        self.add_v(v)
    }
}

impl Sub<Vector, Vector> for Vector {
    fn sub(&self, v : &Vector) -> Vector {
        self.sub_v(v)
    }
}

impl Mul<f32, Vector> for Vector {
    fn mul(&self, s : &f32) -> Vector {
        self.mul_s(*s)
    }
}

impl Mul<Vector, Vector> for f32 {
    fn mul(&self, v : &Vector) -> Vector {
        v.mul_s(*self)
    }
}

impl Div<f32, Vector> for Vector {
    fn div(&self, s : &f32) -> Vector {
        self.div_s(*s)
    }
}

impl Neg<Vector> for Vector {
    fn neg(&self) -> Vector {
        self.reverse()
    }
}

#[test]
fn test_accessors() {
    assert_eq!(Vector::new(1f32, 2f32, 3f32).x, 1f32);
    assert_eq!(Vector::new(1f32, 2f32, 3f32).y, 2f32);
    assert_eq!(Vector::new(1f32, 2f32, 3f32).z, 3f32);
}

#[test]
fn test_equality() {
    assert!(Vector::zero() == Vector::zero());
    assert!(Vector::zero() == Vector::new(0f32, 0f32, 0f32));
    assert!(Vector::zero() != Vector::new(1f32, 0f32, 0f32));
    assert!(Vector::zero() != Vector::new(0f32, 1f32, 0f32));
    assert!(Vector::zero() != Vector::new(0f32, 0f32, 1f32));
    assert!(Vector::unit_x() == Vector::unit_x());
    assert!(Vector::unit_x() != Vector::unit_y());
}

#[test]
fn test_dot() {
    assert_eq!(Vector::new(1f32, 2f32, 3f32).dot(&Vector::zero()), 0f32);
    assert_eq!(Vector::new(1f32, 2f32, 3f32).dot(&Vector::unit_y()), 2f32);
    assert_eq!(Vector::new(1f32, 2f32, 3f32).dot(&Vector::new(4f32, 5f32, 6f32)), 32f32);
}

#[test]
fn test_cross() {
    assert_eq!(Vector::unit_x().cross(&Vector::unit_y()), Vector::unit_z());
}

#[test]
fn test_magnitude() {
    assert_eq!(Vector::zero().magnitude(), 0f32);
    assert_eq!(Vector::unit_x().magnitude(), 1f32);
}

#[test]
fn test_normalize() {
    assert_eq!(Vector::unit_x().mul_s(3f32).normalize(), Vector::unit_x());
}

#[test]
fn test_reverse() {
    assert_eq!(Vector::zero().reverse(), Vector::zero());
    assert_eq!(Vector::new(1f32, -2f32, 3f32).reverse(), Vector::new(-1f32, 2f32, -3f32));
}

#[test]
fn test_add() {
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_x()), Vector::new(2f32, 0f32, 0f32));
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_y()), Vector::new(1f32, 1f32, 0f32));
    assert_eq!(Vector::unit_x().add_v(&Vector::unit_z()), Vector::new(1f32, 0f32, 1f32));

    let mut v = Vector::unit_x();
    v.add_self_v(&Vector::unit_x());
    v.add_self_v(&Vector::unit_y());
    assert_eq!(v, Vector::new(2f32, 1f32, 0f32));
}

#[test]
fn test_mul() {
    assert_eq!(Vector::unit_x().mul_s(3f32), Vector::new(3f32, 0f32, 0f32));
    assert_eq!(Vector::unit_y().mul_s(3f32), Vector::new(0f32, 3f32, 0f32));

    let mut v = Vector::unit_x();
    v.mul_self_s(3f32);
    assert_eq!(v, Vector::new(3f32, 0f32, 0f32));
}

#[test]
fn test_div() {
    assert_eq!(Vector::unit_x().mul_s(3f32).div_s(3f32), Vector::unit_x());
    assert_eq!(Vector::unit_y().mul_s(3f32).div_s(3f32), Vector::unit_y());

    let mut v = Vector::unit_x();
    v.mul_self_s(3f32);
    v.div_self_s(3f32);
    assert_eq!(v, Vector::unit_x());
}

#[test]
fn test_angle() {
    assert_eq!(Vector::unit_x().angle_between(&Vector::unit_y()), Float::frac_pi_2());
    assert_eq!(Vector::unit_y().angle_between(&Vector::unit_x()), Float::frac_pi_2());
}
