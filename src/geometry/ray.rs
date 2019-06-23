use std::fmt::{Display,Formatter,Result};
use std::ops::{Add,Sub,Neg};

use crate::geometry::transform::{HasTransform};
use crate::geometry::vector::Vector;
use crate::geometry::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin    : Point,
    pub direction : Vector,
}

impl Ray {
    pub fn new(origin : &Point, direction : &Vector) -> Ray {
        Ray { origin: *origin, direction: *direction }
    }

    pub fn x_axis() -> Ray {
        Ray::new(&Point::origin(), &Vector::unit_x())
    }

    pub fn y_axis() -> Ray {
        Ray::new(&Point::origin(), &Vector::unit_y())
    }

    pub fn z_axis() -> Ray {
        Ray::new(&Point::origin(), &Vector::unit_z())
    }

    pub fn at_time(&self, t : f32) -> Point {
        self.origin + (self.direction * t)
    }

    pub fn reverse(&self) -> Ray {
        Ray::new(&self.origin, &-self.direction)
    }

    pub fn reverse_self(&mut self) {
        self.direction.reverse_self()
    }

    pub fn to<T : HasTransform>(&self, t : &T) -> Ray {
        Ray::new(&self.origin.to(t), &self.direction.to(t))
    }

    pub fn from<T : HasTransform>(&self, t : &T) -> Ray {
        Ray::new(&self.origin.from(t), &self.direction.from(t))
    }
}

impl Display for Ray {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "Ray {{ origin: {}, direction: {} }}", self.origin, self.direction)
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }

    fn ne(&self, other: &Ray) -> bool {
        self.origin != other.origin || self.direction != other.direction
    }
}

impl Add<Vector> for Ray {
    type Output=Ray;
    fn add(self, v : Vector) -> Ray {
        Ray::new(&self.origin, &(self.direction + v))
    }
}

impl Sub<Vector> for Ray {
    type Output=Ray;
    fn sub(self, v : Vector) -> Ray {
        Ray::new(&self.origin, &(self.direction - v))
    }
}

impl Neg for Ray {
    type Output=Ray;
    fn neg(self) -> Ray {
        self.reverse()
    }
}

#[test]
fn test_equality() {
    assert!(Ray::x_axis() == Ray::x_axis());
    assert!(Ray::x_axis() != Ray::y_axis());
    assert!(Ray::x_axis() == Ray::new(&Point::origin(), &Vector::unit_x()));
}

#[test]
fn test_reverse() {
    assert_eq!(Ray::x_axis().reverse(), Ray::new(&Point::origin(), &Vector::unit_x().reverse()));
    assert_eq!(Ray::x_axis().reverse().reverse(), Ray::x_axis());

    let mut r = Ray::x_axis();
    r.reverse_self();
    assert_eq!(r, Ray::new(&Point::origin(), &Vector::unit_x().reverse()));
    r.reverse_self();
    assert_eq!(r, Ray::x_axis());
}

#[test]
fn test_at_time() {
    assert_eq!(Ray::x_axis().at_time(3f32), Point::new(3f32, 0f32, 0f32));
}
