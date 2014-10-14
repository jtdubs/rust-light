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
        o - *self
    }

    pub fn vector_from(&self, o : &Point) -> Vector {
        self - *o
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
}

impl Add<Vector, Point> for Point {
    fn add(&self, o : &Vector) -> Point {
        Point::new(self.x+o.x, self.y+o.y, self.z+o.z)
    }
}

impl Sub<Point, Vector> for Point {
    fn sub(&self, o : &Point) -> Vector {
        Vector::new(o.x-self.x, o.y-self.y, o.z-self.z)
    }
}
