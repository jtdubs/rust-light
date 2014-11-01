use matrix::Matrix;
use vector::Vector;
use quaternion::Quaternion;

pub struct Transform {
    m : Matrix,
    n : Matrix,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform { m: Matrix::identity(), n: Matrix::identity() }
    }

    pub fn translation(v : &Vector) -> Transform {
        Transform { m: Matrix::translation(v), n: Matrix::translation(&v.reverse()) }
    }

    pub fn scaling(v : &Vector) -> Transform {
        Transform { m: Matrix::scaling(v), n: Matrix::scaling(&Vector::new(1f32/v.x, 1f32/v.y, 1f32/v.z)) }
    }

    pub fn rotation_q(q : &Quaternion) -> Transform {
        Transform { m: q.to_matrix(), n: q.conjugate().to_matrix() }
    }

    pub fn rotation(angle : f32, axis : &Vector) -> Transform {
        Transform::rotation_q(&Quaternion::rotation(angle, axis))
    }

    pub fn rotation3(pitch : f32, yaw : f32, roll : f32) -> Transform {
        Transform::rotation_q(&Quaternion::rotation3(pitch, yaw, roll))
    }

    pub fn inverse(&self) -> Transform {
        Transform { m: self.n, n: self.m }
    }

    pub fn inverse_self(&mut self) {
        let temp = self.m;
        self.m = self.n;
        self.n = temp;
    }

    pub fn compose(&self, t : &Transform) -> Transform {
        Transform { m: self.m.mul_m(&t.m), n: t.n.mul_m(&self.n) }
    }

    pub fn compose_self(&mut self, t : &Transform) {
        self.m.mul_self_m(&t.m);

        let n = self.n.clone();
        self.n.clone_from(&t.n);
        self.n.mul_self_m(&n);
    }

    pub fn transformation_matrix(&self) -> &Matrix {
        &self.m
    }

    pub fn inverse_transformation_matrix(&self) -> &Matrix {
        &self.n
    }
}

pub trait Trans {
    fn transform(&self, t : &Transform) -> Self;

    fn translate(&self, v : &Vector) -> Self {
        self.transform(&Transform::translation(v))
    }

    fn scale(&self, v : &Vector) -> Self {
        self.transform(&Transform::scaling(v))
    }

    fn rotate_q(&self, q : &Quaternion) -> Self {
        self.transform(&Transform::rotation_q(q))
    }

    fn rotate(&self, angle : f32, axis : &Vector) -> Self {
        self.transform(&Transform::rotation(angle, axis))
    }

    fn rotate3(&self, pitch : f32, yaw : f32, roll : f32) -> Self {
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

// TODO: test inverse
// TODO: do a big transform test
