use std::default::Default;
use std::ops::{Add,Sub,Mul,Div,Neg};
use std::fmt::{Display,Formatter,Result};

use crate::geometry::vector::Vector;
use crate::geometry::transform::{Transform,Trans,TransMut};

#[derive(Copy, Clone)]
pub struct Normal {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Normal {
    pub fn new(x : f32, y : f32, z : f32) -> Normal {
        Normal { x: x, y: y, z: z }
    }

    pub fn zero() -> Normal {
        Normal::new(0f32, 0f32, 0f32)
    }

    pub fn unit_x() -> Normal {
        Normal::new(1f32, 0f32, 0f32)
    }

    pub fn unit_y() -> Normal {
        Normal::new(0f32, 1f32, 0f32)
    }

    pub fn unit_z() -> Normal {
        Normal::new(0f32, 0f32, 1f32)
    }

    pub fn dot(&self, o : &Normal) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Normal {
        let m = self.magnitude();
        if m == 0f32 { *self } else { self.div_s(m) }
    }

    pub fn normalize_self(&mut self) {
        let m = self.magnitude();
        if m != 0f32 { self.div_self_s(m) }
    }

    pub fn face_forward(&self, forward : &Vector) -> Normal {
      if self.dot(&forward.to_normal()) < 0f32 { self.reverse() } else { *self }
    }

    pub fn face_forward_self(&mut self, forward : &Vector) {
        if self.dot(&forward.to_normal()) < 0f32 { self.reverse_self() }
    }

    pub fn reverse(&self) -> Normal {
        Normal::new(-self.x, -self.y, -self.z)
    }

    pub fn reverse_self(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn mul_s(&self, s : f32) -> Normal {
        Normal::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mul_self_s(&mut self, s : f32) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s;
    }

    pub fn div_s(&self, s : f32) -> Normal {
        Normal::new(self.x / s, self.y / s, self.z / s)
    }

    pub fn div_self_s(&mut self, s : f32) {
        self.x = self.x / s;
        self.y = self.y / s;
        self.z = self.z / s;
    }

    pub fn add_n(&self, o : &Normal) -> Normal {
        Normal::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn add_self_n(&mut self, o : &Normal) {
        self.x = self.x + o.x;
        self.y = self.y + o.y;
        self.z = self.z + o.z;
    }
    
    pub fn sub_n(&self, o : &Normal) -> Normal {
        Normal::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn sub_self_n(&mut self, o : &Normal) {
        self.x = self.x - o.x;
        self.y = self.y - o.y;
        self.z = self.z - o.z;
    }
}

impl Display for Normal {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl PartialEq for Normal {
    fn eq(&self, other: &Normal) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Normal) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

impl Default for Normal {
    fn default() -> Normal {
        Normal::new(0f32, 0f32, 0f32)
    }
}

impl Trans for Normal {
    type Output=Normal;

    fn transform(&self, t : &Transform) -> Normal {
        t.inverse_transformation_matrix().transpose().mul_n(self)
    }
}

impl TransMut for Normal {
    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.inverse_transformation_matrix().transpose().mul_n(&c))
    }
}

impl Add<Normal> for Normal {
    type Output = Normal;
    fn add(self, n : Normal) -> Normal {
        self.add_n(&n)
    }
}

impl Sub<Normal> for Normal {
    type Output = Normal;
    fn sub(self, n : Normal) -> Normal {
        self.sub_n(&n)
    }
}

impl Mul<f32> for Normal {
    type Output = Normal;
    fn mul(self, s : f32) -> Normal {
        self.mul_s(s)
    }
}

impl Mul<Normal> for f32 {
    type Output = Normal;
    fn mul(self, n : Normal) -> Normal {
        n.mul_s(self)
    }
}

impl Div<f32> for Normal {
    type Output = Normal;
    fn div(self, s : f32) -> Normal {
        self.div_s(s)
    }
}

impl Neg for Normal {
    type Output = Normal;
    fn neg(self) -> Normal {
        self.reverse()
    }
}

#[test]
fn test_accessors() {
    assert_eq!(Normal::new(1f32, 2f32, 3f32).x, 1f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).y, 2f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).z, 3f32);
}

#[test]
fn test_construction() {
    assert_eq!(Normal::new(1f32, 2f32, 3f32), Vector::new(1f32, 2f32, 3f32).to_normal());
    assert_eq!(Normal::new(1f32, 2f32, 3f32).x, 1f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).y, 2f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).z, 3f32);
}

#[test]
fn test_equality() {
    assert!(Normal::zero() == Normal::zero());
    assert!(Normal::zero() == Normal::new(0f32, 0f32, 0f32));
    assert!(Normal::zero() != Normal::new(1f32, 0f32, 0f32));
    assert!(Normal::zero() != Normal::new(0f32, 1f32, 0f32));
    assert!(Normal::zero() != Normal::new(0f32, 0f32, 1f32));
    assert!(Normal::unit_x() == Normal::unit_x());
    assert!(Normal::unit_x() != Normal::unit_y());
}

#[test]
fn test_dot() {
    assert_eq!(Normal::new(1f32, 2f32, 3f32).dot(&Normal::zero()), 0f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).dot(&Normal::unit_y()), 2f32);
    assert_eq!(Normal::new(1f32, 2f32, 3f32).dot(&Normal::new(4f32, 5f32, 6f32)), 32f32);
}

#[test]
fn test_magnitude() {
    assert_eq!(Normal::zero().magnitude(), 0f32);
    assert_eq!(Normal::unit_x().magnitude(), 1f32);
}

#[test]
fn test_normalize() {
    assert_eq!(Normal::unit_x().mul_s(3f32).normalize(), Normal::unit_x());
}

#[test]
fn test_reverse() {
    assert_eq!(Normal::zero().reverse(), Normal::zero());
    assert_eq!(Normal::new(1f32, -2f32, 3f32).reverse(), Normal::new(-1f32, 2f32, -3f32));
}

#[test]
fn test_add() {
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_x()), Normal::new(2f32, 0f32, 0f32));
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_y()), Normal::new(1f32, 1f32, 0f32));
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_z()), Normal::new(1f32, 0f32, 1f32));

    let mut v = Normal::unit_x();
    v.add_self_n(&Normal::unit_x());
    v.add_self_n(&Normal::unit_y());
    assert_eq!(v, Normal::new(2f32, 1f32, 0f32));
}

#[test]
fn test_mul() {
    assert_eq!(Normal::unit_x().mul_s(3f32), Normal::new(3f32, 0f32, 0f32));
    assert_eq!(Normal::unit_y().mul_s(3f32), Normal::new(0f32, 3f32, 0f32));

    let mut v = Normal::unit_x();
    v.mul_self_s(3f32);
    assert_eq!(v, Normal::new(3f32, 0f32, 0f32));
}

#[test]
fn test_div() {
    assert_eq!(Normal::unit_x().mul_s(3f32).div_s(3f32), Normal::unit_x());
    assert_eq!(Normal::unit_y().mul_s(3f32).div_s(3f32), Normal::unit_y());

    let mut v = Normal::unit_x();
    v.mul_self_s(3f32);
    v.div_self_s(3f32);
    assert_eq!(v, Normal::unit_x());
}

#[test]
fn face_forward() {
    assert_eq!(Normal::unit_x().reverse().face_forward(&Vector::unit_x()), Normal::unit_x());
}
