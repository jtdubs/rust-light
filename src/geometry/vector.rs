use std::default::Default;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Display, Formatter, Result};

use crate::geometry::{Matrix, Normal, HasTransform};

#[derive(Copy, Clone, Debug)]
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
        if m == 0f32 { *self } else { *self / m }
    }

    pub fn normalize_self(&mut self) {
        let m = self.magnitude();
        if m != 0f32 { self.div_self_s(m) }
    }

    pub fn angle_between(&self, o : &Vector) -> f32 {
        (self.dot(o) / (self.magnitude() * o.magnitude())).acos()
    }

    pub fn to_normal(&self) -> Normal {
        Normal::new(self.x, self.y, self.z).normalize()
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

    pub fn to<T : HasTransform>(&self, t : &T) -> Vector {
        t.get_transform().to_object.mul_v(self)
    }

    pub fn from<T : HasTransform>(&self, t : &T) -> Vector {
        t.get_transform().to_world.mul_v(self)
    }
}

impl Display for Vector {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() < 1e-3f32 && (self.y - other.y).abs() < 1e-3f32 && (self.z - other.z).abs() < 1e-3f32
    }

    fn ne(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() > 1e-3f32 || (self.y - other.y).abs() > 1e-3f32 || (self.z - other.z).abs() > 1e-3f32
    }
}

impl Default for Vector {
    fn default() -> Vector {
        Vector::zero()
    }
}

impl Add<Vector> for Vector {
    type Output=Vector;
    fn add(self, v : Vector) -> Vector {
        self.add_v(&v)
    }
}

impl Sub<Vector> for Vector {
    type Output=Vector;
    fn sub(self, v : Vector) -> Vector {
        self.sub_v(&v)
    }
}

impl Mul<f32> for Vector {
    type Output=Vector;
    fn mul(self, s : f32) -> Vector {
        self.mul_s(s)
    }
}

impl Mul<Vector> for f32 {
    type Output=Vector;
    fn mul(self, v : Vector) -> Vector {
        v.mul_s(self)
    }
}

impl Div<f32> for Vector {
    type Output=Vector;
    fn div(self, s : f32) -> Vector {
        self.div_s(s)
    }
}

impl Neg for Vector {
    type Output=Vector;
    fn neg(self) -> Vector {
        self.reverse()
    }
}

impl Mul<Matrix> for Vector {
    type Output = Vector;
    fn mul(self, m : Matrix) -> Vector {
        m.premul_v(&self)
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::*;
    use super::*;

    use quickcheck::*;

    impl Arbitrary for Vector {
        fn arbitrary<G: Gen>(g : &mut G) -> Vector {
            Vector::new(f32::arbitrary(g), f32::arbitrary(g), f32::arbitrary(g))
        }
    }

    #[quickcheck]
    fn prop_constructor_x_is_x(x : f32) -> bool { x == Vector::new(x, 0f32, 0f32).x  }

    #[quickcheck]
    fn prop_constructor_y_is_y(y : f32) -> bool { y == Vector::new(0f32, y, 0f32).y }

    #[quickcheck]
    fn prop_constructor_z_is_z(z : f32) -> bool { z == Vector::new(0f32, 0f32, z).z }

    #[quickcheck]
    fn prop_vector_equals_itself(x : f32, y : f32, z : f32) -> bool { Vector::new(x, y, z) == Vector::new(x, y, z) }

    #[quickcheck]
    fn prop_dot_unit_x_is_x(v : Vector) -> bool { v.dot(&Vector::unit_x()) == v.x }

    #[quickcheck]
    fn prop_dot_unit_y_is_y(v : Vector) -> bool { v.dot(&Vector::unit_y()) == v.y }

    #[quickcheck]
    fn prop_dot_unit_z_is_z(v : Vector) -> bool { v.dot(&Vector::unit_z()) == v.z }

    #[quickcheck]
    fn prop_reverse_reverse_is_identity(v : Vector) -> bool { v.reverse().reverse() == v }

    #[quickcheck]
    fn prop_zero_is_additive_identity(v : Vector) -> bool { v + Vector::zero() == v }

    #[quickcheck]
    fn prop_one_is_multiplicitive_identity(v : Vector) -> bool { v * 1f32 == v }

    #[quickcheck]
    fn prop_zero_is_multiplicitive_zero(v : Vector) -> bool { v * 0f32 == Vector::zero() }

    #[quickcheck]
    fn prop_normalize_is_length_one(v : Vector) -> bool { (v.normalize().magnitude() - 1f32).abs() <= 1e-4f32 }

    #[quickcheck]
    fn prop_addition_commutes(a : Vector, b : Vector) -> bool { a + b == b + a }

    #[quickcheck]
    fn prop_addition_associates(a : Vector, b : Vector, c : Vector) -> bool { a + (b + c) == (a + b) + c }

    #[quickcheck]
    fn prop_division_cancels_multiplication(s : f32, a : Vector) -> bool { s == 0f32 || a * s / s == a }

    #[quickcheck]
    fn prop_one_is_division_identity(a : Vector) -> bool { a / 1f32 == a }

    #[quickcheck]
    fn prop_negate_negate_is_identity(a : Vector) -> bool { -(-a) == a }

    #[quickcheck]
    fn prop_subtraction_is_negated_addition(a : Vector, b : Vector) -> bool { a - b == a + -b }

    #[test]
    fn test_cross() {
        assert_eq!(Vector::unit_x().cross(&Vector::unit_y()), Vector::unit_z());
    }

    #[test]
    fn test_angle() {
        assert_eq!(Vector::unit_x().angle_between(&Vector::unit_y()), FRAC_PI_2);
        assert_eq!(Vector::unit_y().angle_between(&Vector::unit_x()), FRAC_PI_2);
    }
}
