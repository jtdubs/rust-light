use geometry::vector::Vector;
use geometry::transform::{Transform,Transformable};

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

impl Transformable for Point {
    fn transform(&self, t : &Transform) -> Point {
        t.transformation_matrix().mul_p(self)
    }

    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.transformation_matrix().mul_p(&c))
    }
}
