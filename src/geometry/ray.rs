use geometry::vector::Vector;
use geometry::point::Point;

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
        self.origin + (self.direction * t)
    }
}

impl Neg<Ray> for Ray {
    fn neg(&self) -> Ray {
        Ray::new(&self.origin, &-self.direction)
    }
}

