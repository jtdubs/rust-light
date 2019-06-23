use std::default::Default;
use std::ops::{Add,Sub,Mul,Div,Index};
use std::fmt::{Display,Formatter,Result};

use crate::geometry::vector::Vector;
use crate::geometry::point::Point;
use crate::geometry::normal::Normal;

#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    m : [f32; 16],
}

impl Matrix {
    pub fn new(m : &[f32; 16]) -> Matrix {
        Matrix { m: *m }
    }

    pub fn zero() -> Matrix {
        Matrix::new(&[0f32; 16])
    }

    pub fn identity() -> Matrix {
        Matrix::new(&[1f32, 0f32, 0f32, 0f32,
                      0f32, 1f32, 0f32, 0f32,
                      0f32, 0f32, 1f32, 0f32,
                      0f32, 0f32, 0f32, 1f32])
    }

    pub fn scaling(v : &Vector) -> Matrix {
        Matrix::new(&[ v.x, 0f32, 0f32, 0f32,
                      0f32,  v.y, 0f32, 0f32,
                      0f32, 0f32,  v.z, 0f32,
                      0f32, 0f32, 0f32, 1f32])
    }
     
    pub fn translation(v : &Vector) -> Matrix {
        Matrix::new(&[1f32, 0f32, 0f32,  v.x,
                      0f32, 1f32, 0f32,  v.y,
                      0f32, 0f32, 1f32,  v.z,
                      0f32, 0f32, 0f32, 1f32])
    }
     
    pub fn rotation(angle : f32, axis : &Vector) -> Matrix {
        let c = angle.cos();
        let s = angle.sin();
        let u = axis.x;
        let v = axis.y;
        let w = axis.z;
        Matrix::new(&[  u*u*(1f32-c)+c, u*v*(1f32-c)-w*s, u*w*(1f32-c)+v*s, 0f32,
                      v*u*(1f32-c)+w*c,   v*v*(1f32-c)+c, u*w*(1f32-c)-u*s, 0f32,
                      w*u*(1f32-c)-v*c, w*v*(1f32-c)+u*s,   u*w*(1f32-c)+c, 0f32,
                                  0f32,             0f32,             0f32, 1f32])
    }
     
    pub fn frustum(l : f32, r : f32, b : f32, t : f32, n : f32, f : f32) -> Matrix {
        let a = (r+l)/(r-l);
        let b2 = (t+b)/(t-b);
        let c = -(f+n)/(f-n);
        let d = -(2f32*f*n)/(f-n);
        Matrix::new(&[(2f32*n)/(r-l),           0f32,     a, 0f32,
                                0f32, (2f32*n)/(t-b),    b2, 0f32,
                                0f32,           0f32,     c,    d,
                                0f32,           0f32, -1f32, 0f32])
    }
     
    pub fn perspective(fov_y : f32, aspect : f32, near : f32, far : f32) -> Matrix {
        let fh = (fov_y / 2f32).tan() * near;
        let fw = fh * aspect;
        Matrix::frustum(-fw, fw, -fh, fh, near, far)
    }
     
    pub fn orthographic(l : f32, r : f32, b : f32, t : f32, n : f32, f : f32) -> Matrix {
        let tx = -(r+l)/(r-l);
        let ty = -(t+b)/(t-b);
        let tz = -(f+n)/(f-n);
        Matrix::new(&[2f32/(r-l),       0f32,        0f32,   tx,
                            0f32, 2f32/(t-b),        0f32,   ty,
                            0f32,       0f32, -2f32*(f-n),   tz,
                            0f32,       0f32,        0f32, 1f32])
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::new(&[self[0], self[4], self[ 8], self[12],
                      self[1], self[5], self[ 9], self[13],
                      self[2], self[6], self[10], self[14],
                      self[3], self[7], self[11], self[15]])
    }

    pub fn transpose_self(&mut self) {
        self.m.swap(1, 4);
        self.m.swap(2, 8);
        self.m.swap(3, 12);
        self.m.swap(6, 9);
        self.m.swap(7, 13);
        self.m.swap(11, 14);
    }

    pub fn mul_m(&self, o : &Matrix) -> Matrix {
        Matrix::new(&[self[ 0] * o.m[ 0] + self[ 1] * o.m[ 4] + self[ 2] * o.m[ 8] + self[ 3] * o.m[12],
                      self[ 0] * o.m[ 1] + self[ 1] * o.m[ 5] + self[ 2] * o.m[ 9] + self[ 3] * o.m[13],
                      self[ 0] * o.m[ 2] + self[ 1] * o.m[ 6] + self[ 2] * o.m[10] + self[ 3] * o.m[14],
                      self[ 0] * o.m[ 3] + self[ 1] * o.m[ 7] + self[ 2] * o.m[11] + self[ 3] * o.m[15],
                      self[ 4] * o.m[ 0] + self[ 5] * o.m[ 4] + self[ 6] * o.m[ 8] + self[ 7] * o.m[12],
                      self[ 4] * o.m[ 1] + self[ 5] * o.m[ 5] + self[ 6] * o.m[ 9] + self[ 7] * o.m[13],
                      self[ 4] * o.m[ 2] + self[ 5] * o.m[ 6] + self[ 6] * o.m[10] + self[ 7] * o.m[14],
                      self[ 4] * o.m[ 3] + self[ 5] * o.m[ 7] + self[ 6] * o.m[11] + self[ 7] * o.m[15],
                      self[ 8] * o.m[ 0] + self[ 9] * o.m[ 4] + self[10] * o.m[ 8] + self[11] * o.m[12],
                      self[ 8] * o.m[ 1] + self[ 9] * o.m[ 5] + self[10] * o.m[ 9] + self[11] * o.m[13],
                      self[ 8] * o.m[ 2] + self[ 9] * o.m[ 6] + self[10] * o.m[10] + self[11] * o.m[14],
                      self[ 8] * o.m[ 3] + self[ 9] * o.m[ 7] + self[10] * o.m[11] + self[11] * o.m[15],
                      self[12] * o.m[ 0] + self[13] * o.m[ 4] + self[14] * o.m[ 8] + self[15] * o.m[12],
                      self[12] * o.m[ 1] + self[13] * o.m[ 5] + self[14] * o.m[ 9] + self[15] * o.m[13],
                      self[12] * o.m[ 2] + self[13] * o.m[ 6] + self[14] * o.m[10] + self[15] * o.m[14],
                      self[12] * o.m[ 3] + self[13] * o.m[ 7] + self[14] * o.m[11] + self[15] * o.m[15]])
    }

    pub fn mul_self_m(&mut self, o : &Matrix) {
        let r = self.mul_m(o);
        self.clone_from(&r);
    }

    pub fn mul_s(&self, s : f32) -> Matrix {
        Matrix::new(&[self[ 0] * s, self[ 1] * s, self[ 2] * s, self[ 3] * s,
                      self[ 4] * s, self[ 5] * s, self[ 6] * s, self[ 7] * s,
                      self[ 8] * s, self[ 9] * s, self[10] * s, self[11] * s,
                      self[12] * s, self[13] * s, self[14] * s, self[15] * s])
    }

    pub fn mul_self_s(&mut self, s : f32) {
        for ix in 0..16 {
            self.m[ix] = self.m[ix] * s
        }
    }

    pub fn div_s(&self, s : f32) -> Matrix {
        Matrix::new(&[self[ 0] / s, self[ 1] / s, self[ 2] / s, self[ 3] / s,
                      self[ 4] / s, self[ 5] / s, self[ 6] / s, self[ 7] / s,
                      self[ 8] / s, self[ 9] / s, self[10] / s, self[11] / s,
                      self[12] / s, self[13] / s, self[14] / s, self[15] / s])
    }

    pub fn div_self_s(&mut self, s : f32) {
        for ix in 0..16 {
            self.m[ix] = self.m[ix] / s
        }
    }

    pub fn add_m(&self, m : &Matrix) -> Matrix {
        Matrix::new(&[self[ 0] + m[ 0], self[ 1] + m[ 1], self[ 2] + m[ 2], self[ 3] + m[ 3],
                      self[ 4] + m[ 4], self[ 5] + m[ 5], self[ 6] + m[ 6], self[ 7] + m[ 7],
                      self[ 8] + m[ 8], self[ 9] + m[ 9], self[10] + m[10], self[11] + m[11],
                      self[12] + m[12], self[13] + m[13], self[14] + m[14], self[15] + m[15]])
    }

    pub fn add_self_m(&mut self, m : &Matrix) {
        for ix in 0..16usize {
            self.m[ix] = self.m[ix] + m[ix]
        }
    }

    pub fn sub_m(&self, m : &Matrix) -> Matrix {
        Matrix::new(&[self[ 0] - m[ 0], self[ 1] - m[ 1], self[ 2] - m[ 2], self[ 3] - m[ 3],
                      self[ 4] - m[ 4], self[ 5] - m[ 5], self[ 6] - m[ 6], self[ 7] - m[ 7],
                      self[ 8] - m[ 8], self[ 9] - m[ 9], self[10] - m[10], self[11] - m[11],
                      self[12] - m[12], self[13] - m[13], self[14] - m[14], self[15] - m[15]])
    }

    pub fn sub_self_m(&mut self, m : &Matrix) {
        for ix in 0..16 {
            self.m[ix] = self.m[ix] - m[ix]
        }
    }

    pub fn mul_v(&self, v : &Vector) -> Vector {
        Vector::new(self[ 0] * v.x + self[ 1] * v.y + self[ 2] * v.z,
                    self[ 4] * v.x + self[ 5] * v.y + self[ 6] * v.z,
                    self[ 8] * v.x + self[ 9] * v.y + self[10] * v.z)
    }

    pub fn premul_v(&self, v : &Vector) -> Vector {
        Vector::new(self[ 0] * v.x + self[ 4] * v.y + self[ 8] * v.z,
                    self[ 1] * v.x + self[ 5] * v.y + self[ 9] * v.z,
                    self[ 2] * v.x + self[ 6] * v.y + self[10] * v.z)
    }

    pub fn mul_n(&self, n : &Normal) -> Normal {
        Normal::new(self[ 0] * n.x + self[ 1] * n.y + self[ 2] * n.z,
                    self[ 4] * n.x + self[ 5] * n.y + self[ 6] * n.z,
                    self[ 8] * n.x + self[ 9] * n.y + self[10] * n.z)
    }

    pub fn premul_n(&self, n : &Normal) -> Normal {
        Normal::new(self[ 0] * n.x + self[ 4] * n.y + self[ 8] * n.z,
                    self[ 1] * n.x + self[ 5] * n.y + self[ 9] * n.z,
                    self[ 2] * n.x + self[ 6] * n.y + self[10] * n.z)
    }
    
    pub fn mul_p(&self, p : &Point) -> Point {
        let s = self[12] * p.x + self[13] * p.y + self[14] * p.z + self[15];
        
        if s == 0f32 {
            Point::origin()
        } else {
            Point::new((self[ 0] * p.x + self[ 1] * p.y + self[ 2] * p.z + self[ 3]) / s,
                       (self[ 4] * p.x + self[ 5] * p.y + self[ 6] * p.z + self[ 7]) / s,
                       (self[ 8] * p.x + self[ 9] * p.y + self[10] * p.z + self[11]) / s)
        }
    }

    pub fn premul_p(&self, p : &Point) -> Point {
        let s = self[3] * p.x + self[7] * p.y + self[11] * p.z + self[15];
        
        if s == 0f32 {
            Point::origin()
        } else {
            Point::new((self[ 0] * p.x + self[ 4] * p.y + self[ 8] * p.z + self[12]) / s,
                       (self[ 1] * p.x + self[ 5] * p.y + self[ 9] * p.z + self[13]) / s,
                       (self[ 2] * p.x + self[ 6] * p.y + self[10] * p.z + self[14]) / s)
        }
    }
}

impl Index<usize> for Matrix {
    type Output = f32;
    fn index(&self, index : usize) -> &Self::Output {
        &self.m[index]
    }
}

impl Display for Matrix {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]",
                 self[ 0], self[ 1], self[ 2], self[ 3],
                 self[ 4], self[ 5], self[ 6], self[ 7],
                 self[ 8], self[ 9], self[10], self[11],
                 self[12], self[13], self[14], self[15])
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.m == other.m
    }

    fn ne(&self, other: &Matrix) -> bool {
        self.m != other.m
    }
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix::zero()
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, m : Matrix) -> Matrix {
        self.add_m(&m)
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, m : Matrix) -> Matrix {
        self.sub_m(&m)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, m : Matrix) -> Matrix {
        self.mul_m(&m)
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;
    fn mul(self, s : f32) -> Matrix {
        self.mul_s(s)
    }
}

impl Mul<Matrix> for f32 {
    type Output = Matrix;
    fn mul(self, m : Matrix) -> Matrix {
        m.mul_s(self)
    }
}

impl Div<f32> for Matrix {
    type Output = Matrix;
    fn div(self, s : f32) -> Matrix {
        self.div_s(s)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, v : Vector) -> Vector {
        self.mul_v(&v)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, p : Point) -> Point {
        self.mul_p(&p)
    }
}

impl Mul<Normal> for Matrix {
    type Output = Normal;
    fn mul(self, n : Normal) -> Normal {
        self.mul_n(&n)
    }
}

#[test]
fn test_access() {
    let l = Matrix::new(&[ 0f32,  1f32,  2f32,  3f32,
                           4f32,  5f32,  6f32,  7f32,
                           8f32,  9f32, 10f32, 11f32,
                          12f32, 13f32, 14f32, 15f32]);

    for ix in 0..16 {
        assert_eq!(l[ix], ix as f32);
    }
}

#[test]
fn test_equality() {
    assert!(Matrix::zero() == Matrix::zero());
    assert!(Matrix::identity() == Matrix::identity());
    assert!(Matrix::identity() != Matrix::zero());
    assert!(Matrix::identity() == Matrix::identity().clone());
}

#[test]
fn test_transpose() {
    let l = Matrix::new(&[ 0f32,  1f32,  2f32,  3f32,
                           4f32,  5f32,  6f32,  7f32,
                           8f32,  9f32, 10f32, 11f32,
                          12f32, 13f32, 14f32, 15f32]);
    let r = Matrix::new(&[ 0f32,   4f32,  8f32, 12f32,
                           1f32,   5f32,  9f32, 13f32,
                           2f32,   6f32, 10f32, 14f32,
                           3f32,   7f32, 11f32, 15f32]);
    assert_eq!(l.transpose(), r);
    assert_eq!(l.transpose().transpose(), l);
}

#[test]
fn test_mul() {
    let l = Matrix::new(&[ 0f32,  1f32,  2f32,  3f32,
                           4f32,  5f32,  6f32,  7f32,
                           8f32,  9f32, 10f32, 11f32,
                          12f32, 13f32, 14f32, 15f32]);

    let r1 = l.mul_m(&Matrix::identity());
    let r2 = Matrix::identity().mul_m(&l);

    assert_eq!(l, r1);
    assert_eq!(l, r2);

    let mut l2 = l.clone();
    l2.mul_self_m(&Matrix::identity());
    assert_eq!(l, l2);
}

#[test]
fn test_mul_div_s() {
    let l1 = Matrix::new(&[ 0f32,  1f32,  2f32,  3f32,
                            4f32,  5f32,  6f32,  7f32,
                            8f32,  9f32, 10f32, 11f32,
                           12f32, 13f32, 14f32, 15f32]);

    let l2 = Matrix::new(&[ 0f32,  2f32,  4f32,  6f32,
                            8f32, 10f32, 12f32, 14f32,
                           16f32, 18f32, 20f32, 22f32,
                           24f32, 26f32, 28f32, 30f32]);

    assert_eq!(l1.mul_s(2f32), l2);

    let mut l1c = l1.clone();

    l1c.mul_self_s(2f32);
    assert_eq!(l1c, l2);
    l1c.div_self_s(2f32);
    assert_eq!(l1c, l1);
}

#[test]
fn test_add_sub() {
    let l1 = Matrix::new(&[ 0f32,  1f32,  2f32,  3f32,
                            4f32,  5f32,  6f32,  7f32,
                            8f32,  9f32, 10f32, 11f32,
                           12f32, 13f32, 14f32, 15f32]);

    let l2 = Matrix::new(&[ 0f32,  2f32,  4f32,  6f32,
                            8f32, 10f32, 12f32, 14f32,
                           16f32, 18f32, 20f32, 22f32,
                           24f32, 26f32, 28f32, 30f32]);

    assert_eq!(l1.add_m(&l1), l2);
    assert_eq!(l2.sub_m(&l1), l1);

    let mut l1c = l1.clone();

    l1c.add_self_m(&l1);
    assert_eq!(l1c, l2);
    l1c.sub_self_m(&l1);
    assert_eq!(l1c, l1);
}

#[test]
fn test_vector_math() {
    let v = Vector::new(1f32, 2f32, 3f32);

    assert_eq!(Matrix::identity().mul_v(&v), v);
    assert_eq!(Matrix::identity().premul_v(&v), v);
}

#[test]
fn test_point_math() {
    let p = Point::new(1f32, 2f32, 3f32);

    assert_eq!(Matrix::identity().mul_p(&p), p);
    assert_eq!(Matrix::identity().premul_p(&p), p);
}

#[test]
fn test_normal_math() {
    let n = Vector::new(1f32, 2f32, 3f32).to_normal();

    assert_eq!(Matrix::identity().mul_n(&n), n);
    assert_eq!(Matrix::identity().premul_n(&n), n);
}
