use geometry::vector::Vector;
use geometry::matrix::Matrix;

#[deriving(Show)]
pub struct Point {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}
 
impl Point {
    pub fn new(x : f64, y : f64, z : f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn origin() -> Point {
        Point::new(0f64, 0f64, 0f64)
    }

    pub fn distance_squared(&self, o : &Point) -> f64 {
        self.vector_to(o).magnitude_squared()
    }

    pub fn distance(&self, o : &Point) -> f64 {
        self.vector_to(o).magnitude()
    }

    pub fn vector_to(&self, o : &Point) -> Vector {
        o.sub_p(self)
    }

    pub fn vector_from(&self, o : &Point) -> Vector {
        self.sub_p(o)
    }

    pub fn mul_m(&self, m : &Matrix) -> Point {
        let s = m[3] * self.x + m[7] * self.y + m[11] * self.z + m[15];
        
        if s == 0f64 {
            Point::origin()
        } else {
            Point::new((m[ 0] * self.x + m[ 4] * self.y + m[ 8] * self.z + m[12]) / s,
                       (m[ 1] * self.x + m[ 5] * self.y + m[ 9] * self.z + m[13]) / s,
                       (m[ 2] * self.x + m[ 6] * self.y + m[10] * self.z + m[14]) / s)
        }
    }

    pub fn mul_self_m(&mut self, m : &Matrix) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let s = m[3] * x + m[7] * y + m[11] * z + m[15];
        if s == 0f64 {
            self.x = 0f64;
            self.y = 0f64;
            self.z = 0f64;
        } else {
            self.x = (m[0] * x + m[4] * y + m[ 8] * z + m[12]) / s;
            self.y = (m[1] * x + m[5] * y + m[ 9] * z + m[13]) / s;
            self.z = (m[2] * x + m[6] * y + m[10] * z + m[14]) / s;
        }
    }

    pub fn add_v(&self, o : &Vector) -> Point {
        Point::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn add_self_v(&mut self, o : &Vector) {
        self.x = self.x + o.x;
        self.y = self.y + o.y;
        self.z = self.z + o.z;
    }

    pub fn sub_v(&self, o : &Vector) -> Point {
        Point::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn sub_self_v(&mut self, o : &Vector) {
        self.x = self.x - o.x;
        self.y = self.y - o.y;
        self.z = self.z - o.z
    }

    pub fn sub_p(&self, o : &Point) -> Vector {
        Vector::new(o.x - self.x, o.y - self.y, o.z - self.z)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }

    fn clone_from(&mut self, source: &Point) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Point) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}
