use geometry::transform::{Transform,Trans,TransMut};
use aabb::AABB;
use geometry::ray::Ray;
use geometry::point::Point;
use shapes::shape::Shape;

pub struct RectangularPrism {
    t : Transform,
    hw : f32,
    hh : f32,
    hd : f32,
}

impl RectangularPrism {
    pub fn new(width : f32, height : f32, depth : f32) -> RectangularPrism {
        RectangularPrism { t: Transform::identity(), hw: width / 2f32, hh: height / 2f32, hd: depth / 2f32 }
    }

    pub fn unit() -> RectangularPrism {
        RectangularPrism::new(1f32, 1f32, 1f32)
    }
}

impl Shape for RectangularPrism {
    fn bound(&self) -> AABB {
        AABB::for_points([Point::new(-self.hw, -self.hh, -self.hd), Point::new(self.hw, self.hh, self.hd)])
    }

    fn surface_area(&self) -> f32 {
        (8f32 * self.hd * self.hw) + (8f32 * self.hd * self.hh) + (8f32 * self.hw * self.hh)
    }

    fn world_bound(&self) -> AABB {
        self.bound().transform(&self.t)
    }

    fn intersections(&self, r : &Ray) -> Vec<f32> {
        let mut res = Vec::new();
        let ray = r.transform(&self.t.inverse());

        let tx1 = (-self.hw - ray.origin.x) / ray.direction.x;
        let tx2 = (self.hw - ray.origin.x) / ray.direction.x;
        let ty1 = (-self.hh - ray.origin.y) / ray.direction.y;
        let ty2 = (self.hh - ray.origin.y) / ray.direction.y;
        let tz1 = (-self.hd - ray.origin.z) / ray.direction.z;
        let tz2 = (self.hd - ray.origin.z) / ray.direction.z;
        let (t0x, t1x) = if tx1 < tx2 { (tx1, tx2) } else { (tx2, tx1) };
        let (t0y, t1y) = if ty1 < ty2 { (ty1, ty2) } else { (ty2, ty1) };
        let (t0z, t1z) = if tz1 < tz2 { (tz1, tz2) } else { (tz2, tz1) };
        
        if t0x <= t1y && t0x <= t1z && t0y <= t1x && t0y <= t1z && t0z <= t1x && t0z <= t1y {
            let t0 = t0x.max(t0y).max(t0z);
            let t1 = t1x.min(t1y).min(t1z);
            
            if t0 >= 0f32 { res.push(t0); }
            if t1 >= 0f32 { res.push(t1); }
        }
        
        res
    }

    fn intersect(&self, r : &Ray) -> Option<f32> {
        let ray = r.transform(&self.t.inverse());

        let tx1 = (-self.hw - ray.origin.x) / ray.direction.x;
        let tx2 = (self.hw - ray.origin.x) / ray.direction.x;
        let ty1 = (-self.hh - ray.origin.y) / ray.direction.y;
        let ty2 = (self.hh - ray.origin.y) / ray.direction.y;
        let tz1 = (-self.hd - ray.origin.z) / ray.direction.z;
        let tz2 = (self.hd - ray.origin.z) / ray.direction.z;
        let (t0x, t1x) = if tx1 < tx2 { (tx1, tx2) } else { (tx2, tx1) };
        let (t0y, t1y) = if ty1 < ty2 { (ty1, ty2) } else { (ty2, ty1) };
        let (t0z, t1z) = if tz1 < tz2 { (tz1, tz2) } else { (tz2, tz1) };
        
        if t0x <= t1y && t0x <= t1z && t0y <= t1x && t0y <= t1z && t0z <= t1x && t0z <= t1y {
            let t0 = t0x.max(t0y).max(t0z);
            let t1 = t1x.min(t1y).min(t1z);
            
            if t0 >= 0f32 { return Some(t0); }
            if t1 >= 0f32 { return Some(t1); }
        }
        
        None
    }
}

impl Trans for RectangularPrism {
    fn transform(&self, t : &Transform) -> RectangularPrism {
        RectangularPrism { t: t.compose(&self.t), hw: self.hw, hh: self.hh, hd: self.hd }
    }
}

impl TransMut for RectangularPrism {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
