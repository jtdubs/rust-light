use std::fmt::{Show,Formatter,Result};

use geometry::vector::Vector;
use geometry::point::Point;
use geometry::transform::{Transform,Transformable};

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

    pub fn at_time(&self, t : f64) -> Point {
        self.origin.add_v(&self.direction.mul_s(t))
    }

    pub fn reverse(&self) -> Ray {
        Ray::new(&self.origin, &self.direction.reverse())
    }

    pub fn reverse_self(&mut self) {
        self.direction.reverse_self()
    }
}

impl Show for Ray {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "Ray {{ origin: {}, direction: {} }}", self.origin, self.direction)
    }
}

impl Transformable for Ray {
    fn transform(&self, t : &Transform) -> Ray {
        Ray::new(&self.origin.transform(t), &self.direction.transform(t))
    }

    fn transform_self(&mut self, t : &Transform) {
        self.origin.transform_self(t);
        self.direction.transform_self(t)
    }
}
