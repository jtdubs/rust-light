use std::num::FloatMath;

use geometry::matrix::Matrix;
use geometry::vector::Vector;

#[deriving(Show)]
pub struct Quaternion {
    v : Vector,
    w : f64
}

impl Quaternion {
    pub fn new(v : &Vector, w : f64) -> Quaternion {
        Quaternion { v: *v, w: w }
    }

    pub fn identity() -> Quaternion {
        Quaternion::new(&Vector::zero(), 1f64)
    }

    pub fn normalize(&self) -> Quaternion {
        let s = self.magnitude_squared();
        Quaternion::new(&self.v.div_s(s), self.w)
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
        Matrix::new([1f64 - 2f64 * (yy + zz),        2f64 * (xy - wz),        2f64 * (xz + wy), 0f64,
                            2f64 * (wy + wz), 1f64 - 2f64 * (xx + zz),        2f64 * (yz - wx), 0f64,
                            2f64 * (xz - wy),        2f64 * (yz + wx), 1f64 - 2f64 * (xx + yy), 0f64,
                                        0f64,                    0f64,                    0f64, 1f64])
    }

    pub fn to_angle_axis(&self) -> (f64, Vector) {
        (self.w.acos() * 2f64, self.v.normalize())
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.v.dot(&self.v) + (self.w * self.w)
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(&self.v.reverse(), self.w)
    }

    pub fn conjugate_self(&mut self) {
        self.v = self.v.reverse();
    }

    pub fn dot(&self, q : &Quaternion) -> f64 {
        return self.v.dot(&q.v) + (self.w * q.w)
    }

    pub fn mul_v(&self, v : &Vector) -> Vector {
        if *v == Vector::zero() {
            *v
        } else {
            self.mul_q(&Quaternion::new(v, 0f64)).mul_q(&self.conjugate()).v
        }
    }

    pub fn mul_q(&self, q : &Quaternion) -> Quaternion {
        Quaternion::new(
            &self.v.cross(&q.v).add_v(&self.v.mul_s(q.w)).add_v(&q.v.mul_s(self.w)),
            self.w*q.w - (self.v.dot(&q.v)))
    }

    // TODO: mul_self_q

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

pub fn rotation_quaternion(angle : f64, axis : &Vector) -> Quaternion {
    Quaternion::new(&axis.normalize().mul_s((angle/2f64).sin()), (angle/2f64).cos())
}

pub fn rotation_quaternion3(pitch : f64, yaw : f64, roll : f64) -> Quaternion {
    let p = pitch/2f64;
    let y = yaw/2f64;
    let r = roll/2f64;
    let sp = p.sin();
    let sy = y.sin();
    let sr = r.sin();
    let cp = p.cos();
    let cy = y.cos();
    let cr = r.cos();
    Quaternion::new(&Vector::new(cr*sp*cy + sr*cp*sy, cr*cp*sy - sr*sp*cy, sr*cp*cy - cr*sp*sy), cr*cp*cy + sr*sp*sy)
}
