use geometry::transform::{Transform,Trans,TransMut};
use aabb::AABB;
use geometry::ray::Ray;
use geometry::point::Point;
use math::quadratic;
use shapes::shape::Shape;

pub struct Paraboloid {
    t : Transform,
    r : f32,
    h : f32,
}

impl Paraboloid {
    pub fn new(diameter : f32, height : f32) -> Paraboloid {
        Paraboloid { t: Transform::identity(), r: diameter / 2f32, h: height }
    }

    pub fn unit() -> Paraboloid {
        Paraboloid::new(1f32, 1f32)
    }
}

impl Shape for Paraboloid {
    fn bound(&self) -> AABB {
        AABB::for_points([Point::new(-self.r, -self.r, 0f32), Point::new(self.r, self.r, self.h)])
    }

    fn world_bound(&self) -> AABB {
        self.bound().transform(&self.t)
    }

    fn surface_area(&self) -> f32 {
        (self.r / (self.h * self.h)) * ((self.r * self.r + 4f32 * self.h * self.h) * 1.5f32 - self.r * self.r * self.r) * Float::frac_pi_6()
    }

    fn intersections(&self, r : &Ray) -> Vec<f32> {
        let mut res = Vec::new();
        let ray = r.transform(&self.t.inverse());

        let a = (self.h * ray.direction.x * ray.direction.x + self.h * ray.direction.y * ray.direction.y) / (self.r * self.r);
        let b = (2f32 * self.h * ray.origin.x * ray.direction.x + 2f32 * self.h * ray.origin.y * ray.direction.y) / (self.r * self.r) - ray.direction.z;
        let c = (self.h * ray.origin.x * ray.origin.x + self.h * ray.origin.y * ray.origin.y) / (self.r * self.r) - ray.origin.z;
        match quadratic(a, b, c) {
            None => { },
            Some((t1, t2)) => {
                let z1 = ray.at_time(t1).z;
                let z2 = ray.at_time(t2).z;
                if t1 >= 0f32 && z1 >= 0f32 && z1 <= self.h { res.push(t1); };
                if t2 >= 0f32 && z2 >= 0f32 && z2 <= self.h { res.push(t2); };
            },
        }
        
        res
    }

    fn intersect(&self, r : &Ray) -> Option<f32> {
        let ray = r.transform(&self.t.inverse());

        let a = (self.h * ray.direction.x * ray.direction.x + self.h * ray.direction.y * ray.direction.y) / (self.r * self.r);
        let b = (2f32 * self.h * ray.origin.x * ray.direction.x + 2f32 * self.h * ray.origin.y * ray.direction.y) / (self.r * self.r) - ray.direction.z;
        let c = (self.h * ray.origin.x * ray.origin.x + self.h * ray.origin.y * ray.origin.y) / (self.r * self.r) - ray.origin.z;
        match quadratic(a, b, c) {
            None => { None },
            Some((t1, t2)) => {
                let z1 = ray.at_time(t1).z;
                if t1 >= 0f32 && z1 >= 0f32 && z1 <= self.h { return Some(t1); }
                let z2 = ray.at_time(t2).z;
                if t2 >= 0f32 && z2 >= 0f32 && z2 <= self.h { return Some(t2); }
                None
            },
        }
    }
}

impl Trans for Paraboloid {
    fn transform(&self, t : &Transform) -> Paraboloid {
        Paraboloid { t: t.compose(&self.t), r: self.r, h: self.h }
    }
}

impl TransMut for Paraboloid {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}

