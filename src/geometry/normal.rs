use geometry::vector::Vector;
use geometry::matrix::Matrix;

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

    pub fn normalize_self(&self) {
        let m = self.magnitude();
        if m != 0f64 { self.div_self_s(m) }
    }

    pub fn face_forward(&self, forward : &Vector) -> Normal {
      if self.dot(&forward.to_normal()) < 0f64 { self.reverse() } else { *self }
    }

    pub fn face_forward_self(&mut self, forward : &Vector) {
        if self.dot(&forward.to_normal()) < 0f64 { self.reverse_self() }
    }

    pub fn mul_m(&self, m : &Matrix) -> Normal {
        Normal::new(m[ 0] * self.x + m[ 4] * self.y + m[ 8] * self.z,
                    m[ 1] * self.x + m[ 5] * self.y + m[ 9] * self.z,
                    m[ 2] * self.x + m[ 6] * self.y + m[10] * self.z)
    }

    pub fn mul_self_m(&mut self, m : &Matrix) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        self.x = m[0] * x + m[4] * y + m[ 8] * z;
        self.y = m[1] * x + m[5] * y + m[ 9] * z;
        self.z = m[2] * x + m[6] * y + m[10] * z;
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
