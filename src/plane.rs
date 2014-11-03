use transform::{Transform,Trans,TransMut};
use aabb::AABB;
use ray::Ray;
use point::Point;
use shape::Shape;

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

impl Shape for Plane {
    fn bound(&self) -> AABB {
        AABB::for_points([Point::new(-self.hw, -self.hd, 0f32), Point::new(self.hw, self.hd, 0f32)])
    }

    fn world_bound(&self) -> AABB {
        self.bound().transform(&self.t)
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.hw * self.hd
    }

    fn intersections(&self, r : &Ray) -> Vec<f32> {
        let mut res = Vec::new();
        let ray = r.transform(&self.t.inverse());

        if ray.direction.z > 0.0001 {
            let t = -ray.origin.z / ray.direction.z;
            let p = ray.at_time(t);
            if t >= 0f32 && p.x.abs() <= self.hw && p.y.abs() <= self.hd { res.push(t); };
        }
        
        res
    }

    fn intersect(&self, r : &Ray) -> Option<f32> {
        let ray = r.transform(&self.t.inverse());

        if ray.direction.z.abs() < 0.0001 { return None; }
        let t = -ray.origin.z / ray.direction.z;
        let p = ray.at_time(t);
        if t >= 0f32 && p.x.abs() <= self.hw && p.y.abs() <= self.hd {
            Some(t)
        } else { 
            None
        }
    }
}

impl Trans for Plane {
    fn transform(&self, t : &Transform) -> Plane {
        Plane { t: t.compose(&self.t), hw: self.hw, hd: self.hd }
    }
}

impl TransMut for Plane {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}