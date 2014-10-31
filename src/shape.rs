use transform::{Transform,Trans,TransMut};
use aabb::AABB;
use ray::Ray;
use point::Point;
use math::quadratic;

pub enum Shape {
    Box(Transform, f64, f64, f64),
    Cone(Transform, f64, f64),
    Cylinder(Transform, f64, f64),
}

impl Shape {
    pub fn new_box(half_width : f64, half_height : f64, half_depth : f64) -> Shape {
        Box(Transform::identity(), half_width, half_height, half_depth)
    }

    pub fn new_cone(radius : f64, height : f64) -> Shape {
        Cone(Transform::identity(), radius, height)
    }

    pub fn new_cylinder(radius : f64, height : f64) -> Shape {
        Cylinder(Transform::identity(), radius, height)
    }

    pub fn new_unit_box() -> Shape {
        Shape::new_box(1f64, 1f64, 1f64)
    }

    pub fn new_unit_cone() -> Shape {
        Shape::new_cone(1f64, 1f64)
    }

    pub fn new_unit_cylinder() -> Shape {
        Shape::new_cylinder(1f64, 1f64)
    }

    pub fn bound(&self) -> AABB {
        match self {
            &Box(_, hw, hh, hd) => AABB::for_points([Point::new(-hw, -hh, -hd), Point::new(hw, hh, hd)]),
            &Cone(_, r, h) => AABB::for_points([Point::new(-r, -r, 0f64), Point::new(r, r, h)]),
            &Cylinder(_, r, h) => AABB::for_points([Point::new(-r, -r, 0f64), Point::new(r, r, h)]),
        }
    }

    fn get_transform(&self) -> &Transform {
        match self {
            &Box(ref t, _, _, _) => t,
            &Cone(ref t, _, _) => t,
            &Cylinder(ref t, _, _) => t,
        }
    }

    pub fn intersections(&self, r : &Ray) -> Vec<f64> {
        let mut res = Vec::new();
        let ray = r.transform(&self.get_transform().inverse());

        match self {
            &Box(_, hw, hh, hd) => {
                let tx1 = (-hw - ray.origin.x) / ray.direction.x;
                let tx2 = (hw - ray.origin.x) / ray.direction.x;
                let ty1 = (-hh - ray.origin.y) / ray.direction.y;
                let ty2 = (hh - ray.origin.y) / ray.direction.y;
                let tz1 = (-hd - ray.origin.z) / ray.direction.z;
                let tz2 = (hd - ray.origin.z) / ray.direction.z;
                let (t0x, t1x) = if tx1 < tx2 { (tx1, tx2) } else { (tx2, tx1) };
                let (t0y, t1y) = if ty1 < ty2 { (ty1, ty2) } else { (ty2, ty1) };
                let (t0z, t1z) = if tz1 < tz2 { (tz1, tz2) } else { (tz2, tz1) };

                if t0x <= t1y && t0x <= t1z && t0y <= t1x && t0y <= t1z && t0z <= t1x && t0z <= t1y {
                    let t0 = t0x.max(t0y).max(t0z);
                    let t1 = t1x.min(t1y).min(t1z);

                    if t0 >= 0f64 { res.push(t0); }
                    if t1 >= 0f64 { res.push(t1); }
                }
            },
            &Cone(_, r, h) => {
                let a = (h * h * ray.direction.x * ray.direction.x + h * h * ray.direction.y * ray.direction.y) / (r * r) + (-ray.direction.z * ray.direction.z);
                let b = (2f64 * h * h * ray.origin.x * ray.origin.x + 2f64 * h * h * ray.origin.y * ray.origin.y) / (r * r) + (-2f64 * ray.origin.z * ray.direction.z + 2f64 * ray.direction.z * h);
                let c = (h * h * ray.origin.x * ray.origin.x + h * h * ray.origin.y * ray.origin.y) / (-ray.origin.z * ray.origin.z + 2f64 * ray.origin.z * h - h * h);
                match quadratic(a, b, c) {
                    None => { }
                    Some([t1, t2]) => {
                        let z1 = ray.at_time(t1).z;
                        let z2 = ray.at_time(t2).z;
                        if t1 >= 0f64 && z1 >= 0f64 && z1 <= h { res.push(t1); };
                        if t2 >= 0f64 && z2 >= 0f64 && z2 <= h { res.push(t2); };
                    }
                }
            },
            &Cylinder(_, r, h) => {
                let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y);
                let b = 2f64 * ((ray.direction.x * ray.origin.x) + (ray.direction.y + ray.origin.y));
                let c = (ray.origin.x * ray.origin.x) + (ray.origin.y * ray.origin.y) - (r * r);
                match quadratic(a, b, c) {
                    None => { },
                    Some([t1, t2]) => {
                        let z1 = ray.at_time(t1).z;
                        let z2 = ray.at_time(t2).z;
                        if t1 >= 0f64 && z1 >= 0f64 && z1 <= h { res.push(t1); };
                        if t2 >= 0f64 && z2 >= 0f64 && z2 <= h { res.push(t2); };
                    },
                }
            },
        };

        res
    }

    pub fn surface_area(&self) -> f64 {
        match self {
            &Box(_, hw, hh, hd) => (8f64 * hd * hw) + (8f64 * hd * hh) + (8f64 * hw * hh),
            &Cone(_, r, h) => r * (r * r + h * h).sqrt() * Float::pi(),
            &Cylinder(_, r, h) => 2f64 * r * h * Float::pi(),
        }
    }

    pub fn world_bound(&self) -> AABB {
        self.bound().transform(self.get_transform())
    }

    pub fn intersect(&self, r : &Ray) -> Option<f64> {
        let ts = self.intersections(r);
        if ts.len() == 0 {
            None
        } else {
            Some(ts[0])
        }
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        let ts = self.intersections(r);
        ts.len() > 0
    }
}

impl Trans for Shape {
    fn transform(&self, t : &Transform) -> Shape {
        match self {
            &Box(c, hw, hh, hd) => Box(t.compose(&c), hw, hh, hd),
            &Cone(c, r, h) => Cone(t.compose(&c), r, h), 
            &Cylinder(c, r, h) => Cylinder(t.compose(&c), r, h),
        }
    }
}

impl TransMut for Shape {
    fn transform_self(&mut self, t : &Transform) {
        match self {
            &Box(ref mut c, _, _, _) => { *c = t.compose(c); },
            &Cone(ref mut c, _, _) => { *c = t.compose(c); },
            &Cylinder(ref mut c, _, _) => { *c = t.compose(c); },
        };
    }
}
