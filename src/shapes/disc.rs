use std::default::Default;

use geometry::transform::{Transform,Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;
use geometry::point::Point;
use shapes::shape::{Shape,Intersection};

pub struct Disc {
    t : Transform,
    r : f32
}

impl Disc {
    pub fn new(diameter : f32) -> Disc {
        Disc { t: Transform::identity(), r: diameter / 2f32 }
    }

    pub fn unit() -> Disc {
        Disc::new(1f32)
    }
}

impl Default for Disc {
    fn default() -> Disc {
        Disc::unit()
    }
}

impl Shape for Disc {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points([Point::new(-self.r, -self.r, 0f32), Point::new(self.r, self.r, 0f32)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound() * self.t
    }

    fn surface_area(&self) -> f32 {
        2f32 * self.r * self.r * Float::pi()
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = r * -self.t;

        if ray.direction.z > 0.0001 {
            let t = -ray.origin.z / ray.direction.z;
            let p = ray.at_time(t);
            let d = p.distance_squared(&Point::origin());
            if t >= 0f32 && d <= (self.r * self.r) { 
                res.push(Intersection::new(r, t, &r.at_time(t)));
            };
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r * -self.t;

        if ray.direction.z.abs() < 0.0001 { return None; }
        let t = -ray.origin.z / ray.direction.z;
        let p = ray.at_time(t);
        let d = p.distance_squared(&Point::origin());
        if t >= 0f32 && d <= (self.r * self.r) { 
            Some(Intersection::new(r, t, &r.at_time(t)))
        } else {
            None
        }
    }
}

impl Trans for Disc {
    fn transform(&self, t : &Transform) -> Disc {
        Disc { t: t + self.t, .. *self }
    }
}

impl TransMut for Disc {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t + self.t;
    }
}

