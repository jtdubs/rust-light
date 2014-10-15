use std::fmt::{Show,Formatter,Result};

use geometry::vector::Vector;
use geometry::transform::{Transform,Transformable};

pub struct Normal {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}
 
impl Normal {
    pub fn new(x : f64, y : f64, z : f64) -> Normal {
        Normal { x: x, y: y, z: z }
    }

    pub fn zero() -> Normal {
        Normal::new(0f64, 0f64, 0f64)
    }

    pub fn unit_x() -> Normal {
        Normal::new(1f64, 0f64, 0f64)
    }

    pub fn unit_y() -> Normal {
        Normal::new(0f64, 1f64, 0f64)
    }

    pub fn unit_z() -> Normal {
        Normal::new(0f64, 0f64, 1f64)
    }

    pub fn dot(&self, o : &Normal) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Normal {
        let m = self.magnitude();
        if m == 0f64 { *self } else { self.div_s(m) }
    }

    pub fn normalize_self(&mut self) {
        let m = self.magnitude();
        if m != 0f64 { self.div_self_s(m) }
    }

    pub fn face_forward(&self, forward : &Vector) -> Normal {
      if self.dot(&forward.to_normal()) < 0f64 { self.reverse() } else { *self }
    }

    pub fn face_forward_self(&mut self, forward : &Vector) {
        if self.dot(&forward.to_normal()) < 0f64 { self.reverse_self() }
    }

    pub fn reverse(&self) -> Normal {
        Normal::new(-self.x, -self.y, -self.z)
    }

    pub fn reverse_self(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn mul_s(&self, s : f64) -> Normal {
        Normal::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mul_self_s(&mut self, s : f64) {
        self.x = self.x * s;
        self.y = self.y * s;
        self.z = self.z * s;
    }

    pub fn div_s(&self, s : f64) -> Normal {
        Normal::new(self.x / s, self.y / s, self.z / s)
    }

    pub fn div_self_s(&mut self, s : f64) {
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

impl Show for Normal {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Clone for Normal {
    fn clone(&self) -> Normal {
        Normal::new(self.x, self.y, self.z)
    }

    fn clone_from(&mut self, source: &Normal) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
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

impl Transformable for Normal {
    fn transform(&self, t : &Transform) -> Normal {
        t.inverse_transformation_matrix().transpose().mul_n(self)
    }

    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.inverse_transformation_matrix().transpose().mul_n(&c))
    }
}

#[test]
fn test_accessors() {
    assert_eq!(Normal::new(1f64, 2f64, 3f64).x, 1f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).y, 2f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).z, 3f64);
}

#[test]
fn test_construction() {
    assert_eq!(Normal::new(1f64, 2f64, 3f64), Vector::new(1f64, 2f64, 3f64).to_normal());
    assert_eq!(Normal::new(1f64, 2f64, 3f64).x, 1f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).y, 2f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).z, 3f64);
}

fn test_equality() {
    assert!(Normal::zero() == Normal::zero());
    assert!(Normal::zero() == Normal::new(0f64, 0f64, 0f64));
    assert!(Normal::zero() != Normal::new(1f64, 0f64, 0f64));
    assert!(Normal::zero() != Normal::new(0f64, 1f64, 0f64));
    assert!(Normal::zero() != Normal::new(0f64, 0f64, 1f64));
    assert!(Normal::unit_x() == Normal::unit_x());
    assert!(Normal::unit_x() != Normal::unit_y());
}

#[test]
fn test_dot() {
    assert_eq!(Normal::new(1f64, 2f64, 3f64).dot(&Normal::zero()), 0f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).dot(&Normal::unit_y()), 2f64);
    assert_eq!(Normal::new(1f64, 2f64, 3f64).dot(&Normal::new(4f64, 5f64, 6f64)), 32f64);
}

#[test]
fn test_magnitude() {
    assert_eq!(Normal::zero().magnitude(), 0f64);
    assert_eq!(Normal::unit_x().magnitude(), 1f64);
}

#[test]
fn test_normalize() {
    assert_eq!(Normal::unit_x().mul_s(3f64).normalize(), Normal::unit_x());
}

#[test]
fn test_reverse() {
    assert_eq!(Normal::zero().reverse(), Normal::zero());
    assert_eq!(Normal::new(1f64, -2f64, 3f64).reverse(), Normal::new(-1f64, 2f64, -3f64));
}

#[test]
fn test_add() {
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_x()), Normal::new(2f64, 0f64, 0f64));
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_y()), Normal::new(1f64, 1f64, 0f64));
    assert_eq!(Normal::unit_x().add_n(&Normal::unit_z()), Normal::new(1f64, 0f64, 1f64));

    let mut v = Normal::unit_x();
    v.add_self_n(&Normal::unit_x());
    v.add_self_n(&Normal::unit_y());
    assert_eq!(v, Normal::new(2f64, 1f64, 0f64));
}

#[test]
fn test_mul() {
    assert_eq!(Normal::unit_x().mul_s(3f64), Normal::new(3f64, 0f64, 0f64));
    assert_eq!(Normal::unit_y().mul_s(3f64), Normal::new(0f64, 3f64, 0f64));

    let mut v = Normal::unit_x();
    v.mul_self_s(3f64);
    assert_eq!(v, Normal::new(3f64, 0f64, 0f64));
}

#[test]
fn test_div() {
    assert_eq!(Normal::unit_x().mul_s(3f64).div_s(3f64), Normal::unit_x());
    assert_eq!(Normal::unit_y().mul_s(3f64).div_s(3f64), Normal::unit_y());

    let mut v = Normal::unit_x();
    v.mul_self_s(3f64);
    v.div_self_s(3f64);
    assert_eq!(v, Normal::unit_x());
}

#[test]
fn face_forward() {
    assert_eq!(Normal::unit_x().reverse().face_forward(&Vector::unit_x()), Normal::unit_x());
}
