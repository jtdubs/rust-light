use std::fmt::{Show,Formatter,Result};

use geometry::vector::Vector;
use geometry::point::Point;
use geometry::normal::Normal;

pub struct Matrix {
    m : [f64, ..16],
}

impl Matrix {
    pub fn new(m : &[f64, ..16]) -> Matrix {
        Matrix { m: *m }
    }

    pub fn zero() -> Matrix {
        Matrix::new(&[0f64, ..16])
    }

    pub fn identity() -> Matrix {
        Matrix::new(&[1f64, 0f64, 0f64, 0f64,
                      0f64, 1f64, 0f64, 0f64,
                      0f64, 0f64, 1f64, 0f64,
                      0f64, 0f64, 0f64, 1f64])
    }

    pub fn scaling(v : &Vector) -> Matrix {
        Matrix::new(&[ v.x, 0f64, 0f64, 0f64,
                      0f64,  v.y, 0f64, 0f64,
                      0f64, 0f64,  v.z, 0f64,
                      0f64, 0f64, 0f64, 1f64])
    }
     
    pub fn translation(v : &Vector) -> Matrix {
        Matrix::new(&[1f64, 0f64, 0f64,  v.x,
                      0f64, 1f64, 0f64,  v.y,
                      0f64, 0f64, 1f64,  v.z,
                      0f64, 0f64, 0f64, 1f64])
    }
     
    pub fn rotation(angle : f64, axis : &Vector) -> Matrix {
        let c = angle.cos();
        let s = angle.sin();
        let u = axis.x;
        let v = axis.y;
        let w = axis.z;
        Matrix::new(&[  u*u*(1f64-c)+c, u*v*(1f64-c)-w*s, u*w*(1f64-c)+v*s, 0f64,
                      v*u*(1f64-c)+w*c,   v*v*(1f64-c)+c, u*w*(1f64-c)-u*s, 0f64,
                      w*u*(1f64-c)-v*c, w*v*(1f64-c)+u*s,   u*w*(1f64-c)+c, 0f64,
                                  0f64,             0f64,             0f64, 1f64])
    }
     
    pub fn frustum(l : f64, r : f64, b : f64, t : f64, n : f64, f : f64) -> Matrix {
        let a = (r+l)/(r-l);
        let b2 = (t+b)/(t-b);
        let c = -(f+n)/(f-n);
        let d = -(2f64*f*n)/(f-n);
        Matrix::new(&[(2f64*n)/(r-l),           0f64,     a, 0f64,
                                0f64, (2f64*n)/(t-b),    b2, 0f64,
                                0f64,           0f64,     c,    d,
                                0f64,           0f64, -1f64, 0f64])
    }
     
    pub fn perspective(fov_y : f64, aspect : f64, near : f64, far : f64) -> Matrix {
        let fh = (fov_y / 2f64).tan() * near;
        let fw = fh * aspect;
        Matrix::frustum(-fw, fw, -fh, fh, near, far)
    }
     
    pub fn orthographic(l : f64, r : f64, b : f64, t : f64, n : f64, f : f64) -> Matrix {
        let tx = -(r+l)/(r-l);
        let ty = -(t+b)/(t-b);
        let tz = -(f+n)/(f-n);
        Matrix::new(&[2f64/(r-l),       0f64,        0f64,   tx,
                            0f64, 2f64/(t-b),        0f64,   ty,
                            0f64,       0f64, -2f64*(f-n),   tz,
                            0f64,       0f64,        0f64, 1f64])
    }


    pub fn transpose(&self) -> Matrix {
        Matrix::new(&[self[0], self[4], self[ 8], self[12],
                      self[1], self[5], self[ 9], self[13],
                      self[2], self[6], self[10], self[14],
                      self[3], self[7], self[11], self[15]])
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

    pub fn mul_s(&self, s : f64) -> Matrix {
        Matrix::new(&[self[ 0] * s, self[ 1] * s, self[ 2] * s, self[ 3] * s,
                      self[ 4] * s, self[ 5] * s, self[ 6] * s, self[ 7] * s,
                      self[ 8] * s, self[ 9] * s, self[10] * s, self[11] * s,
                      self[12] * s, self[13] * s, self[14] * s, self[15] * s])
    }

    pub fn mul_self_s(&mut self, s : f64) {
        for ix in range(0u, 16u) {
            self.m[ix] = self.m[ix] * s
        }
    }

    pub fn div_s(&self, s : f64) -> Matrix {
        Matrix::new(&[self[ 0] / s, self[ 1] / s, self[ 2] / s, self[ 3] / s,
                      self[ 4] / s, self[ 5] / s, self[ 6] / s, self[ 7] / s,
                      self[ 8] / s, self[ 9] / s, self[10] / s, self[11] / s,
                      self[12] / s, self[13] / s, self[14] / s, self[15] / s])
    }

    pub fn div_self_s(&mut self, s : f64) {
        for ix in range(0u, 16u) {
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
        for ix in range(0u, 16u) {
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
        for ix in range(0u, 16u) {
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
        
        if s == 0f64 {
            Point::origin()
        } else {
            Point::new((self[ 0] * p.x + self[ 1] * p.y + self[ 2] * p.z + self[ 3]) / s,
                       (self[ 4] * p.x + self[ 5] * p.y + self[ 6] * p.z + self[ 7]) / s,
                       (self[ 8] * p.x + self[ 9] * p.y + self[10] * p.z + self[11]) / s)
        }
    }

    pub fn premul_p(&self, p : &Point) -> Point {
        let s = self[3] * p.x + self[7] * p.y + self[11] * p.z + self[15];
        
        if s == 0f64 {
            Point::origin()
        } else {
            Point::new((self[ 0] * p.x + self[ 4] * p.y + self[ 8] * p.z + self[12]) / s,
                       (self[ 1] * p.x + self[ 5] * p.y + self[ 9] * p.z + self[13]) / s,
                       (self[ 2] * p.x + self[ 6] * p.y + self[10] * p.z + self[14]) / s)
        }
    }
}

impl Index<uint, f64> for Matrix {
    fn index(&self, index : &uint) -> &f64 {
        &self.m[*index]
    }
}

impl Show for Matrix {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]\n[{}\t{}\t{}\t{}]",
                 self[ 0], self[ 1], self[ 2], self[ 3],
                 self[ 4], self[ 5], self[ 6], self[ 7],
                 self[ 8], self[ 9], self[10], self[11],
                 self[12], self[13], self[14], self[15])
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Matrix {
        Matrix::new(&self.m)
    }

    fn clone_from(&mut self, source: &Matrix) {
        for ix in range(0u, 16u) {
            self.m[ix] = source.m[ix]
        }
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

#[test]
fn test_access() {
    let l = Matrix::new(&[ 0f64,  1f64,  2f64,  3f64,
                           4f64,  5f64,  6f64,  7f64,
                           8f64,  9f64, 10f64, 11f64,
                          12f64, 13f64, 14f64, 15f64]);

    for ix in range(0u, 16u) {
        assert_eq!(l[ix], ix as f64);
    }
}

fn test_equality() {
    assert!(Matrix::zero() == Matrix::zero());
    assert!(Matrix::identity() == Matrix::identity());
    assert!(Matrix::identity() != Matrix::zero());
    assert!(Matrix::identity() == Matrix::identity().clone());
}

fn test_transpose() {
    let l = Matrix::new(&[ 0f64,  1f64,  2f64,  3f64,
                           4f64,  5f64,  6f64,  7f64,
                           8f64,  9f64, 10f64, 11f64,
                          12f64, 13f64, 14f64, 15f64]);
    let r = Matrix::new(&[ 0f64,   4f64,  8f64, 12f64,
                           1f64,   5f64,  9f64, 13f64,
                           2f64,   6f64, 10f64, 14f64,
                           3f64,   7f64, 11f64, 15f64]);
    assert_eq!(l.transpose(), r);
    assert_eq!(l.transpose().transpose(), l);
}
