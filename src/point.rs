use std::fmt::{Show,Formatter,Result};

use vector::Vector;
use transform::{Transform,Trans,TransMut};

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
        Vector::new(self.x - o.x, self.y - o.y, self.z - o.z)
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

impl Show for Point {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "({}, {}, {})", self.x, self.y, self.z)
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

impl Trans for Point {
    fn transform(&self, t : &Transform) -> Point {
        t.transformation_matrix().mul_p(self)
    }
}

impl TransMut for Point {
    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&t.transformation_matrix().mul_p(&c))
    }
}

#[test]
fn test_accessors() {
    assert_eq!(Point::new(1f64, 2f64, 3f64).x, 1f64);
    assert_eq!(Point::new(1f64, 2f64, 3f64).y, 2f64);
    assert_eq!(Point::new(1f64, 2f64, 3f64).z, 3f64);
}

#[test]
fn test_equality() {
    assert!(Point::origin() == Point::origin());
    assert!(Point::origin() == Point::new(0f64, 0f64, 0f64));
    assert!(Point::origin() != Point::new(1f64, 0f64, 0f64));
    assert!(Point::origin() != Point::new(0f64, 1f64, 0f64));
    assert!(Point::origin() != Point::new(0f64, 0f64, 1f64));
    assert!(Point::new(1f64, 0f64, 0f64) == Point::new(1f64, 0f64, 0f64));
    assert!(Point::new(1f64, 0f64, 0f64) != Point::new(0f64, 1f64, 0f64));
}

#[test]
fn test_distance() {
    let o = Point::origin();
    let d1 = o.add_v(&Vector::unit_x());
    let d2 = d1.add_v(&Vector::unit_y());

    assert_eq!(o.distance(&o), 0f64);
    assert_eq!(o.distance(&d1), 1f64);
    assert_eq!(d1.distance(&d2), 1f64);
    assert_eq!(o.distance(&d2), 2f64.sqrt());
}

#[test]
fn test_vector_to_from() {
    let o = Point::origin();
    let d1 = o.add_v(&Vector::unit_x());
    let d2 = d1.add_v(&Vector::unit_y());

    assert_eq!(o.vector_to(&d1), Vector::unit_x());
    assert_eq!(d1.vector_to(&d2), Vector::unit_y());
    assert_eq!(o.vector_to(&d2), Vector::unit_x().add_v(&Vector::unit_y()));

    assert_eq!(o.vector_from(&d1), Vector::unit_x().reverse());
    assert_eq!(d1.vector_from(&d2), Vector::unit_y().reverse());
    assert_eq!(o.vector_from(&d2), Vector::unit_x().add_v(&Vector::unit_y()).reverse());
}

#[test]
fn test_add() {
    assert_eq!(Point::new(1f64, 0f64, 0f64).add_v(&Vector::unit_x()), Point::new(2f64, 0f64, 0f64));
    assert_eq!(Point::new(1f64, 0f64, 0f64).add_v(&Vector::unit_y()), Point::new(1f64, 1f64, 0f64));
    assert_eq!(Point::new(1f64, 0f64, 0f64).add_v(&Vector::unit_z()), Point::new(1f64, 0f64, 1f64));

    let mut v = Point::new(1f64, 0f64, 0f64);
    v.add_self_v(&Vector::unit_x());
    v.add_self_v(&Vector::unit_y());
    assert_eq!(v, Point::new(2f64, 1f64, 0f64));
}

#[test]
fn test_sub() {
    assert_eq!(Point::new(1f64, 0f64, 0f64).sub_v(&Vector::unit_x()), Point::new(0f64, 0f64, 0f64));
    assert_eq!(Point::new(1f64, 0f64, 0f64).sub_v(&Vector::unit_y()), Point::new(1f64, -1f64, 0f64));
    assert_eq!(Point::new(1f64, 0f64, 0f64).sub_v(&Vector::unit_z()), Point::new(1f64, 0f64, -1f64));

    let mut v = Point::new(1f64, 0f64, 0f64);
    v.sub_self_v(&Vector::unit_x());
    v.sub_self_v(&Vector::unit_y());
    assert_eq!(v, Point::new(0f64, -1f64, 0f64));
}

#[test]
fn test_sub_p() {
    assert_eq!(Point::new(1f64, 0f64, 0f64).sub_p(&Point::origin()), Vector::unit_x());
    assert_eq!(Point::new(1f64, 0f64, 0f64).sub_p(&Point::new(1f64, 1f64, 1f64)), Vector::new(0f64, -1f64, -1f64));
}
