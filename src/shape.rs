use geometry::transform::{Transform,Trans,TransMut};
use geometry::aabb::AABB;
use geometry::ray::Ray;
use geometry::point::Point;

pub trait Shape<'a> : Trans {
    fn get_transform(&'a self) -> &'a Transform;
    fn bound(&self) -> AABB;
    fn intersections(&self, r : &Ray) -> Vec<f64>;
    fn surface_area(&self) -> f64;

    fn world_bound(&'a self) -> AABB {
        self.bound().transform(self.get_transform())
    }

    fn intersect(&self, r : &Ray) -> Option<f64> {
        let ts = self.intersections(r);
        if ts.len() == 0 {
            None
        } else {
            Some(ts[0])
        }
    }

    fn intersects(&self, r : &Ray) -> bool {
        let ts = self.intersections(r);
        if ts.len() == 0 {
            true
        } else {
            false
        }
    }
}

pub trait ShapeMut<'a> : Shape<'a> + TransMut {
}

pub struct Box {
    t : Transform,
    hw : f64,
    hh : f64,
    hd : f64
}

impl Box {
    pub fn new(hw : f64, hh : f64, hd : f64) -> Box {
        Box { t: Transform::identity(), hw: hw, hh: hh, hd: hd }
    }

    pub fn unit() -> Box {
        Box::new(1f64, 1f64, 1f64)
    }
}

impl Trans for Box {
    fn transform(&self, t : &Transform) -> Box {
        Box { t: t.compose(&self.t), hw: self.hw, hh: self.hh, hd: self.hd }
    }
}

impl TransMut for Box {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}

impl<'a> Shape<'a> for Box {
    fn get_transform(&'a self) -> &'a Transform {
        &self.t
    }

    fn bound(&self) -> AABB {
        AABB::for_points([Point::new(-self.hw, -self.hh, -self.hd), Point::new(self.hw, self.hh, self.hd)])
    }

    fn surface_area(&self) -> f64 {
        (8f64*self.hd*self.hw) + (8f64*self.hd*self.hh) + (8f64*self.hw*self.hh)
    }

    fn intersections(&self, r : &Ray) -> Vec<f64> {
        let mut res = Vec::new();

        let r2 = r.transform(&self.get_transform().inverse());
        let tx1 = (-self.hw - r2.origin.x) / r2.direction.x;
        let tx2 = (self.hw - r2.origin.x) / r2.direction.x;
        let ty1 = (-self.hh - r2.origin.y) / r2.direction.y;
        let ty2 = (self.hh - r2.origin.y) / r2.direction.y;
        let tz1 = (-self.hd - r2.origin.z) / r2.direction.z;
        let tz2 = (self.hd - r2.origin.z) / r2.direction.z;
        let (t0x, t1x) = if tx1 < tx2 { (tx1, tx2) } else { (tx2, tx1) };
        let (t0y, t1y) = if ty1 < ty2 { (ty1, ty2) } else { (ty2, ty1) };
        let (t0z, t1z) = if tz1 < tz2 { (tz1, tz2) } else { (tz2, tz1) };

        if t0x <= t1y && t0x <= t1z && t0y <= t1x && t0y <= t1z && t0z <= t1x && t0z <= t1y {
            let t0 = t0x.max(t0y).max(t0z);
            let t1 = t1x.min(t1y).min(t1z);

            if t0 >= 0f64 { res.push(t0); }
            if t1 >= 0f64 { res.push(t1); }
        }
        res
    }
}
