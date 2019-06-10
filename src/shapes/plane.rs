use std::default::Default;

use crate::geometry::transform::{Transform,Trans,TransMut};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::shapes::shape::{Shape,Intersection};

pub struct Plane {
    t : Transform,
    hw : f32,
    hd : f32,
}

impl Plane {
    pub fn new(width : f32, depth : f32) -> Plane {
        Plane { t: Transform::identity(), hw: width / 2f32, hd: depth / 2f32 }
    }

    pub fn unit() -> Plane {
        Plane::new(1f32, 1f32)
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane::unit()
    }
}

impl Shape for Plane {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.hw, -self.hd, 0f32), Point::new(self.hw, self.hd, 0f32)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().transform(&self.t)
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.hw * self.hd
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = r.transform(&-self.t);

        if ray.direction.z > 0.0001 {
            let t = -ray.origin.z / ray.direction.z;
            let p = ray.at_time(t);
            if t >= 0f32 && p.x.abs() <= self.hw && p.y.abs() <= self.hd { 
                res.push(Intersection::new(r, t, &r.at_time(t)));
            };
        }
        
        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r.transform(&-self.t);

        if ray.direction.z.abs() < 0.0001 { return None; }
        let t = -ray.origin.z / ray.direction.z;
        let p = ray.at_time(t);
        if t >= 0f32 && p.x.abs() <= self.hw && p.y.abs() <= self.hd {
            Some(Intersection::new(r, t, &r.at_time(t)))
        } else { 
            None
        }
    }
}

impl Trans for Plane {
    type Output=Plane;

    fn transform(&self, t : &Transform) -> Plane {
        Plane { t: *t + self.t, .. *self }
    }
}

impl TransMut for Plane {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
