use geometry::transform::{Transform,Trans,TransMut};
use aabb::AABB;
use geometry::ray::Ray;
use geometry::point::Point;
use shapes::shape::Shape;

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

impl Shape for Disc {
    fn bound(&self) -> AABB {
        AABB::for_points([Point::new(-self.r, -self.r, 0f32), Point::new(self.r, self.r, 0f32)])
    }

    fn world_bound(&self) -> AABB {
        self.bound().transform(&self.t)
    }

    fn surface_area(&self) -> f32 {
        2f32 * self.r * self.r * Float::pi()
    }

    fn intersections(&self, r : &Ray) -> Vec<f32> {
        let mut res = Vec::new();
        let ray = r.transform(&self.t.inverse());

        if ray.direction.z > 0.0001 {
            let t = -ray.origin.z / ray.direction.z;
            let d = ray.at_time(t).distance_squared(&Point::origin());
            if t >= 0f32 && d <= (self.r * self.r) { res.push(t); };
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<f32> {
        let ray = r.transform(&self.t.inverse());

        if ray.direction.z.abs() < 0.0001 { return None; }
        let t = -ray.origin.z / ray.direction.z;
        let d = ray.at_time(t).distance_squared(&Point::origin());
        if t >= 0f32 && d <= (self.r * self.r) { 
            Some(t)
        } else {
            None
        }
    }
}

impl Trans for Disc {
    fn transform(&self, t : &Transform) -> Disc {
        Disc { t: t.compose(&self.t), r: self.r }
    }
}

impl TransMut for Disc {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}

