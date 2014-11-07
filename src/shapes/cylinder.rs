use std::default::Default;

use geometry::transform::{Transform,Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;
use geometry::point::Point;
use math::quadratic;
use shapes::shape::{Shape,Intersection};

pub struct Cylinder {
    t : Transform,
    r : f32,
    hh : f32,
}

impl Cylinder {
    pub fn new(diameter : f32, height : f32) -> Cylinder {
        Cylinder { t: Transform::identity(), r: diameter / 2f32, hh: height / 2f32 }
    }

    pub fn unit() -> Cylinder {
        Cylinder::new(1f32, 1f32)
    }
}

impl Default for Cylinder {
    fn default() -> Cylinder {
        Cylinder::unit()
    }
}

impl Shape for Cylinder {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points([Point::new(-self.r, -self.r, -self.hh), Point::new(self.r, self.r, self.hh)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound() * self.t
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.r * self.hh * Float::pi()
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = r * -self.t;

        let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y);
        let b = 2f32 * ((ray.direction.x * ray.origin.x) + (ray.direction.y * ray.origin.y));
        let c = (ray.origin.x * ray.origin.x) + (ray.origin.y * ray.origin.y) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { },
            Some((t1, t2)) => {
                let p1 = ray.at_time(t1);
                let z1 = p1.z;
                if t1 >= 0f32 && z1 >= -self.hh && z1 <= self.hh {
                    res.push(Intersection::new(r, t1, &r.at_time(t1)));
                };
                let p2 = ray.at_time(t2);
                let z2 = p2.z;
                if t2 >= 0f32 && z2 >= -self.hh && z2 <= self.hh { 
                    res.push(Intersection::new(r, t2, &r.at_time(t2)));
                };
            },
        }
        
        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r * -self.t;

        let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y);
        let b = 2f32 * ((ray.direction.x * ray.origin.x) + (ray.direction.y * ray.origin.y));
        let c = (ray.origin.x * ray.origin.x) + (ray.origin.y * ray.origin.y) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { None },
            Some((t1, t2)) => {
                let p1 = ray.at_time(t1);
                let z1 = p1.z;
                if t1 >= 0f32 && z1 >= -self.hh && z1 <= self.hh {
                    return Some(Intersection::new(r, t1, &r.at_time(t1)));
                };
                let p2 = ray.at_time(t2);
                let z2 = p2.z;
                if t2 >= 0f32 && z2 >= -self.hh && z2 <= self.hh { 
                    return Some(Intersection::new(r, t2, &r.at_time(t2)));
                };
                None
            },
        }
    }
}

impl Trans for Cylinder {
    fn transform(&self, t : &Transform) -> Cylinder {
        Cylinder { t: t + self.t, .. *self }
    }
}

impl TransMut for Cylinder {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t + self.t;
    }
}

#[test]
fn test_cylinder() {
    let c = Cylinder::unit().rotate3(Float::frac_pi_2(), 0f32, 0f32).translate(&Vector::new(0f32, 0f32, 10f32));
    let r = Ray::z_axis();

    match c.intersect(&r) {
        None => assert!(false),
        Some(t) => assert_eq!(t, 9.5f32),
    }
}
