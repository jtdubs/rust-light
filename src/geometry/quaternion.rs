use std::fmt::{Show,Formatter,Result};
use std::num::FloatMath;

use geometry::matrix::Matrix;
use geometry::vector::Vector;

pub struct Quaternion {
    v : Vector,
    w : f32
}

impl Quaternion {
    pub fn new(v : &Vector, w : f32) -> Quaternion {
        Quaternion { v: *v, w: w }
    }

    pub fn identity() -> Quaternion {
        Quaternion::new(&Vector::zero(), 1f32)
    }

    pub fn rotation(angle : f32, axis : &Vector) -> Quaternion {
        Quaternion::new(&axis.normalize().mul_s((angle/2f32).sin()), (angle/2f32).cos())
    }
     
    pub fn rotation3(pitch : f32, yaw : f32, roll : f32) -> Quaternion {
        let p = pitch/2f32;
        let y = yaw/2f32;
        let r = roll/2f32;
        let sp = p.sin();
        let sy = y.sin();
        let sr = r.sin();
        let cp = p.cos();
        let cy = y.cos();
        let cr = r.cos();
        Quaternion::new(&Vector::new(cr*sp*cy + sr*cp*sy, cr*cp*sy - sr*sp*cy, sr*cp*cy - cr*sp*sy), cr*cp*cy + sr*sp*sy)
    }

    pub fn normalize(&self) -> Quaternion {
        let s = self.magnitude_squared();
        Quaternion::new(&self.v.div_s(s), self.w)
    }

    pub fn normalize_self(&mut self) {
        let s = self.magnitude_squared();
        self.v.div_self_s(s);
    }

    pub fn to_matrix(&self) -> Matrix {
        let x = self.v.x;
        let y = self.v.y;
        let z = self.v.z;
        let w = self.w;
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Matrix::new(&[1f32 - 2f32 * (yy + zz),        2f32 * (xy - wz),        2f32 * (xz + wy), 0f32,
                             2f32 * (wy + wz), 1f32 - 2f32 * (xx + zz),        2f32 * (yz - wx), 0f32,
                             2f32 * (xz - wy),        2f32 * (yz + wx), 1f32 - 2f32 * (xx + yy), 0f32,
                                         0f32,                    0f32,                    0f32, 1f32])
    }

    pub fn to_angle_axis(&self) -> (f32, Vector) {
        (self.w.acos() * 2f32, self.v.normalize())
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.v.dot(&self.v) + (self.w * self.w)
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(&self.v.reverse(), self.w)
    }

    pub fn conjugate_self(&mut self) {
        self.v = self.v.reverse();
    }

    pub fn dot(&self, q : &Quaternion) -> f32 {
        return self.v.dot(&q.v) + (self.w * q.w)
    }

    pub fn mul_v(&self, v : &Vector) -> Vector {
        if *v == Vector::zero() {
            *v
        } else {
            self.mul_q(&Quaternion::new(v, 0f32)).mul_q(&self.conjugate()).v
        }
    }

    pub fn mul_q(&self, q : &Quaternion) -> Quaternion {
        Quaternion::new(
            &self.v.cross(&q.v).add_v(&self.v.mul_s(q.w)).add_v(&q.v.mul_s(self.w)),
            self.w*q.w - (self.v.dot(&q.v)))
    }

    pub fn mul_self_q(&mut self, q : &Quaternion) {
        let vxv = self.v.cross(&q.v);
        let vdv = self.v.dot(&q.v);
        self.v.mul_self_s(q.w);
        self.v.add_self_v(&q.v.mul_s(self.w));
        self.v.add_self_v(&vxv);
        self.w = self.w * q.w - vdv;
    }

    pub fn add_q(&self, o : &Quaternion) -> Quaternion {
        Quaternion::new(&self.v.add_v(&o.v), self.w + o.w)
    }

    pub fn add_self_q(&mut self, o : &Quaternion) {
        self.v.add_self_v(&o.v);
        self.w = self.w + o.w;
    }

    pub fn sub_q(&self, o : &Quaternion) -> Quaternion {
        Quaternion::new(&self.v.sub_v(&o.v), self.w - o.w)
    }

    pub fn sub_self_q(&mut self, o : &Quaternion) {
        self.v.sub_self_v(&o.v);
        self.w = self.w - o.w;
    }
}

impl Show for Quaternion {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "{{{}, {}, {}, {}}}", self.v.x, self.v.y, self.v.z, self.w)
    }
}

impl Clone for Quaternion {
    fn clone(&self) -> Quaternion {
        Quaternion::new(&self.v, self.w)
    }

    fn clone_from(&mut self, source: &Quaternion) {
        self.v = source.v;
        self.w = source.w;
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        self.v == other.v && self.w == other.w
    }

    fn ne(&self, other: &Quaternion) -> bool {
        self.v != other.v || self.w != other.w
    }
}

// TODO: test that the _self methods get the same result as the non-_self methods
