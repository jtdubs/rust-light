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
        self.x*o.x + self.y*o.y + self.z*o.z
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Normal {
        let m = self.magnitude();
        if m == 0f64 { *self } else { self / m }
    }

    pub fn face_forward(&self, forward : &Vector) -> Normal {
      if self.dot(&forward.to_normal()) < 0f64 { -self } else { *self }
    }

    pub fn mul_m(&self, m : &Matrix) -> Normal {
        Normal::new(m[ 0] * self.x + m[ 4] * self.y + m[ 8] * self.z,
                    m[ 1] * self.x + m[ 5] * self.y + m[ 9] * self.z,
                    m[ 2] * self.x + m[ 6] * self.y + m[10] * self.z)
    }
}

impl Neg<Normal> for Normal {
    fn neg(&self) -> Normal {
        Normal::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64, Normal> for Normal {
    fn mul(&self, s : &f64) -> Normal {
        Normal::new(self.x * *s, self.y * *s, self.z * *s)
    }
}

impl Div<f64, Normal> for Normal {
    fn div(&self, s : &f64) -> Normal {
        Normal::new(self.x / *s, self.y / *s, self.z / *s)
    }
}

impl Add<Normal, Normal> for Normal {
    fn add(&self, o : &Normal) -> Normal {
        Normal::new(self.x+o.x, self.y+o.y, self.z+o.z)
    }
}

impl Sub<Normal, Normal> for Normal {
    fn sub(&self, o : &Normal) -> Normal {
        Normal::new(self.x-o.x, self.y-o.y, self.z-o.z)
    }
}
