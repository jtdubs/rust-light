use std::default::Default;

use geometry::transform::{Transform,Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;
use geometry::point::Point;
use math::quadratic;
use shapes::shape::{Shape,Intersection};

pub struct Cone {
    t : Transform,
    r : f32,
    h : f32,
}

impl Cone {
    pub fn new(diameter : f32, height : f32) -> Cone {
        Cone { t: Transform::identity(), r: diameter / 2f32, h: height }
    }

    pub fn unit() -> Cone {
        Cone::new(1f32, 1f32)
    }
}

impl Default for Cone {
    fn default() -> Cone {
        Cone::unit()
    }
}

impl Shape for Cone {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points([Point::new(-self.r, -self.r, 0f32), Point::new(self.r, self.r, self.h)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound() * self.t
    }

    fn surface_area(&self) -> f32 {
        self.r * (self.r * self.r + self.h * self.h).sqrt() * Float::pi()
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = (*r) * -self.t;

        let a = (self.h * self.h * ray.direction.x * ray.direction.x + self.h * self.h * ray.direction.y * ray.direction.y) / (self.r * self.r) + (-ray.direction.z * ray.direction.z);
        let b = (2f32 * self.h * self.h * ray.origin.x * ray.direction.x + 2f32 * self.h * self.h * ray.origin.y * ray.direction.y) / (self.r * self.r) + (-2f32 * ray.origin.z * ray.direction.z + 2f32 * ray.direction.z * self.h);
        let c = (self.h * self.h * ray.origin.x * ray.origin.x + self.h * self.h * ray.origin.y * ray.origin.y) / (self.r * self.r) + (-ray.origin.z * ray.origin.z + 2f32 * ray.origin.z * self.h - self.h * self.h);
        match quadratic(a, b, c) {
            None => { }
            Some((t1, t2)) => {
                let p1 = ray.at_time(t1);
                let p2 = ray.at_time(t2);
                if t1 >= 0f32 && p1.z >= 0f32 && p1.z <= self.h {
                    res.push(Intersection::new(r, t1, &r.at_time(t1))); 
                };
                if t2 >= 0f32 && p2.z >= 0f32 && p2.z <= self.h {
                    res.push(Intersection::new(r, t2, &r.at_time(t2))); 
                };
            }
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = (*r) * -self.t;

        let a = (self.h * self.h * ray.direction.x * ray.direction.x + self.h * self.h * ray.direction.y * ray.direction.y) / (self.r * self.r) + (-ray.direction.z * ray.direction.z);
        let b = (2f32 * self.h * self.h * ray.origin.x * ray.direction.x + 2f32 * self.h * self.h * ray.origin.y * ray.direction.y) / (self.r * self.r) + (-2f32 * ray.origin.z * ray.direction.z + 2f32 * ray.direction.z * self.h);
        let c = (self.h * self.h * ray.origin.x * ray.origin.x + self.h * self.h * ray.origin.y * ray.origin.y) / (self.r * self.r) + (-ray.origin.z * ray.origin.z + 2f32 * ray.origin.z * self.h - self.h * self.h);
        match quadratic(a, b, c) {
            None => { None }
            Some((t1, t2)) => {
                let p1 = ray.at_time(t1);
                let z1 = p1.z;
                if t1 >= 0f32 && z1 >= 0f32 && z1 <= self.h {
                    return Some(Intersection::new(r, t1, &r.at_time(t1)));
                }
                let p2 = ray.at_time(t2);
                let z2 = p2.z;
                if t2 >= 0f32 && z2 >= 0f32 && z2 <= self.h { 
                    return Some(Intersection::new(r, t2, &r.at_time(t2)));
                }
                None
            }
        }
    }
}

impl Trans for Cone {
    fn transform(&self, t : &Transform) -> Cone {
        Cone { t: *t + self.t, .. *self }
    }
}

impl TransMut for Cone {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
