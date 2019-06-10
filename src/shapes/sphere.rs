use std::default::Default;

use crate::geometry::transform::{Transform,Trans,TransMut};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::math::quadratic;
use crate::shapes::shape::{Shape,Intersection};

pub struct Sphere {
    t : Transform,
    r : f32
}

impl Sphere {
    pub fn new(diameter : f32) -> Sphere {
        Sphere { t: Transform::identity(), r: diameter / 2f32 }
    }

    pub fn unit() -> Sphere {
        Sphere::new(1f32)
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere::unit()
    }
}

impl Shape for Sphere {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.r, -self.r, -self.r), Point::new(self.r, self.r, self.r)])
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.r * self.r * core::f32::consts::PI
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().transform(&self.t)
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = r.transform(&-self.t);

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { },
            Some((t1, t2)) => {
                if t1 >= 0f32 { res.push(Intersection::new(r, t1, &r.at_time(t1))); };
                if t2 >= 0f32 { res.push(Intersection::new(r, t2, &r.at_time(t2))); };
            },
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r.transform(&-self.t);

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { None },
            Some((t1, t2)) => {
                if t1 >= 0f32 { return Some(Intersection::new(r, t1, &r.at_time(t1))); }
                if t2 >= 0f32 { return Some(Intersection::new(r, t2, &r.at_time(t2))); }
                None
            },
        }
    }
}

impl Trans for Sphere {
    type Output=Sphere;

    fn transform(&self, t : &Transform) -> Sphere {
        Sphere { t: *t + self.t, .. *self }
    }
}

impl TransMut for Sphere {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
