use std::default::Default;
use std::ops::{Add,Neg};

use crate::geometry::matrix::Matrix;
use crate::geometry::vector::Vector;
use crate::geometry::quaternion::Quaternion;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub to_world : Matrix,
    pub to_object : Matrix,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform { to_world: Matrix::identity(), to_object: Matrix::identity() }
    }

    pub fn translation(v : &Vector) -> Transform {
        Transform { to_world: Matrix::translation(v), to_object: Matrix::translation(&v.reverse()) }
    }

    pub fn scaling(v : &Vector) -> Transform {
        Transform { to_world: Matrix::scaling(v), to_object: Matrix::scaling(&Vector::new(1f32 / v.x, 1f32 / v.y, 1f32 / v.z)) }
    }

    pub fn rotation_q(q : &Quaternion) -> Transform {
        Transform { to_world: q.to_matrix(), to_object: q.conjugate().to_matrix() }
    }

    pub fn rotation(angle : f32, axis : &Vector) -> Transform {
        Transform::rotation_q(&Quaternion::rotation(angle, axis))
    }

    pub fn rotation3(pitch : f32, yaw : f32, roll : f32) -> Transform {
        Transform::rotation_q(&Quaternion::rotation3(pitch, yaw, roll))
    }

    pub fn inverse(&self) -> Transform {
        Transform { to_world: self.to_object, to_object: self.to_world }
    }

    pub fn inverse_self(&mut self) {
        std::mem::swap(&mut self.to_world, &mut self.to_object);
    }

    // TODO: add look-at transform
    // https://stackoverflow.com/a/6802424
    // should be able to reverse order of multiplication and transpose to get the inverse...

    pub fn compose(&self, t : &Transform) -> Transform {
        Transform { to_world: self.to_world * t.to_world, to_object: t.to_object * self.to_object }
    }

    pub fn compose_self(&mut self, t : &Transform) {
        self.to_world.mul_self_m(&t.to_world);

        let tmp = self.to_object.clone();
        self.to_object.clone_from(&t.to_object);
        self.to_object.mul_self_m(&tmp);
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::identity()
    }
}

impl Neg for Transform {
    type Output=Transform;

    fn neg(self) -> Transform {
        self.inverse()
    }
}

impl Add<Transform> for Transform {
    type Output=Transform;

    fn add(self, t : Transform) -> Transform {
        self.compose(&t)
    }
}

pub trait Trans {
    type Output;

    fn transform(&self, t : &Transform) -> Self::Output;

    fn translate(&self, v : &Vector) -> Self::Output {
        self.transform(&Transform::translation(v))
    }

    fn scale(&self, v : &Vector) -> Self::Output {
        self.transform(&Transform::scaling(v))
    }

    fn rotate_q(&self, q : &Quaternion) -> Self::Output {
        self.transform(&Transform::rotation_q(q))
    }

    fn rotate(&self, angle : f32, axis : &Vector) -> Self::Output {
        self.transform(&Transform::rotation(angle, axis))
    }

    fn rotate3(&self, pitch : f32, yaw : f32, roll : f32) -> Self::Output {
        self.transform(&Transform::rotation3(pitch, yaw, roll))
    }
}

pub trait TransMut {
    fn transform_self(&mut self, t : &Transform);

    fn translate_self(&mut self, v : &Vector) {
        self.transform_self(&Transform::translation(v))
    }

    fn scale_self(&mut self, v : &Vector) {
        self.transform_self(&Transform::scaling(v))
    }

    fn rotate_self_q(&mut self, q : &Quaternion) {
        self.transform_self(&Transform::rotation_q(q))
    }

    fn rotate_self(&mut self, angle : f32, axis : &Vector) {
        self.transform_self(&Transform::rotation(angle, axis))
    }

    fn rotate3_self(&mut self, pitch : f32, yaw : f32, roll : f32) {
        self.transform_self(&Transform::rotation3(pitch, yaw, roll))
    }
}
