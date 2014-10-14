use geometry::vector::Vector;
use geometry::transform::{Transform,Transformable};

#[deriving(Show)]
pub struct Normal {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}
 
impl Normal {
    pub fn new(x : f64, y : f64, z : f64) -> Normal {
        Normal { x: x, y: y, z: z }
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
