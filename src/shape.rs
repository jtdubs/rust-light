use transform::{Transform,Trans,TransMut};
use aabb::AABB;
use ray::Ray;
use point::Point;
use vector::Vector;
use math::quadratic;

pub enum Shape {
    Box(Transform, f32, f32, f32),
    Cone(Transform, f32, f32),
    Cylinder(Transform, f32, f32),
    Disc(Transform, f32),
    Plane(Transform, f32, f32),
    Sphere(Transform, f32),
    Triangle(Transform, Point, Point, Point),
    Paraboloid(Transform, f32, f32),
}

// TODO: would like Paraboloid and Cone to be centered on the origin

impl Shape {
    pub fn new_box(width : f32, height : f32, depth : f32) -> Shape {
        Box(Transform::identity(), width / 2f32, height / 2f32, depth / 2f32)
    }

    pub fn new_cone(diameter : f32, height : f32) -> Shape {
        Cone(Transform::identity(), diameter / 2f32, height)
    }

    pub fn new_cylinder(diameter : f32, height : f32) -> Shape {
        Cylinder(Transform::identity(), diameter / 2f32, height / 2f32)
    }

    pub fn new_disc(diameter : f32) -> Shape {
        Disc(Transform::identity(), diameter / 2f32)
    }

    pub fn new_plane(width : f32, depth : f32) -> Shape {
        Plane(Transform::identity(), width / 2f32, depth / 2f32)
    }

    pub fn new_sphere(diameter : f32) -> Shape {
        Sphere(Transform::identity(), diameter / 2f32)
    }

    pub fn new_triangle(a : &Point, b : &Point, c : &Point) -> Shape {
        Triangle(Transform::identity(), a.clone(), b.clone(), c.clone())
    }

    pub fn new_paraboloid(diameter : f32, height : f32) -> Shape {
        Paraboloid(Transform::identity(), diameter / 2f32, height)
    }

    pub fn new_unit_box() -> Shape {
        Shape::new_box(1f32, 1f32, 1f32)
    }

    pub fn new_unit_cone() -> Shape {
        Shape::new_cone(1f32, 1f32)
    }

    pub fn new_unit_cylinder() -> Shape {
        Shape::new_cylinder(1f32, 1f32)
    }

    pub fn new_unit_disc() -> Shape {
        Shape::new_disc(1f32)
    }

    pub fn new_unit_plane() -> Shape {
        Shape::new_plane(1f32, 1f32)
    }

    pub fn new_unit_sphere() -> Shape {
        Shape::new_sphere(1f32)
    }

    pub fn new_unit_triangle() -> Shape {
        Shape::new_triangle(
            &Point::origin().sub_v(&Vector::unit_x().div_s(2f32)).sub_v(&Vector::unit_y().div_s(2f32)),
            &Point::origin().add_v(&Vector::unit_x().div_s(2f32)).sub_v(&Vector::unit_y().div_s(2f32)),
            &Point::origin().add_v(&Vector::unit_y().div_s(2f32)))
    }

    pub fn new_unit_paraboloid() -> Shape {
        Shape::new_paraboloid(1f32, 1f32)
    }

    pub fn bound(&self) -> AABB {
        match self {
            &Box(_, hw, hh, hd) => AABB::for_points([Point::new(-hw, -hh, -hd), Point::new(hw, hh, hd)]),
            &Cone(_, r, h) => AABB::for_points([Point::new(-r, -r, 0f32), Point::new(r, r, h)]),
            &Cylinder(_, r, hh) => AABB::for_points([Point::new(-r, -r, -hh), Point::new(r, r, hh)]),
            &Disc(_, r) => AABB::for_points([Point::new(-r, -r, 0f32), Point::new(r, r, 0f32)]),
            &Plane(_, hw, hd) => AABB::for_points([Point::new(-hw, -hd, 0f32), Point::new(hw, hd, 0f32)]),
            &Sphere(_, r) => AABB::for_points([Point::new(-r, -r, -r), Point::new(r, r, r)]),
            &Triangle(_, a, b, c) => AABB::for_points([a, b, c]),
            &Paraboloid(_, r, h) => AABB::for_points([Point::new(-r, -r, 0f32), Point::new(r, r, h)]),
        }
    }

    fn get_transform(&self) -> &Transform {
        match self {
            &Box(ref t, _, _, _) => t,
            &Cone(ref t, _, _) => t,
            &Cylinder(ref t, _, _) => t,
            &Disc(ref t, _) => t,
            &Plane(ref t, _, _) => t,
            &Sphere(ref t, _) => t,
            &Triangle(ref t, _, _, _) => t,
            &Paraboloid(ref t, _, _) => t,
        }
    }

    pub fn intersections(&self, r : &Ray) -> Vec<f32> {
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

                    if t0 >= 0f32 { res.push(t0); }
                    if t1 >= 0f32 { res.push(t1); }
                }
            },
            &Cone(_, r, h) => {
                let a = (h * h * ray.direction.x * ray.direction.x + h * h * ray.direction.y * ray.direction.y) / (r * r) + (-ray.direction.z * ray.direction.z);
                let b = (2f32 * h * h * ray.origin.x * ray.direction.x + 2f32 * h * h * ray.origin.y * ray.direction.y) / (r * r) + (-2f32 * ray.origin.z * ray.direction.z + 2f32 * ray.direction.z * h);
                let c = (h * h * ray.origin.x * ray.origin.x + h * h * ray.origin.y * ray.origin.y) / (r * r) + (-ray.origin.z * ray.origin.z + 2f32 * ray.origin.z * h - h * h);
                match quadratic(a, b, c) {
                    None => { }
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        let z2 = ray.at_time(t2).z;
                        if t1 >= 0f32 && z1 >= 0f32 && z1 <= h { res.push(t1); };
                        if t2 >= 0f32 && z2 >= 0f32 && z2 <= h { res.push(t2); };
                    }
                }
            },
            &Cylinder(_, r, hh) => {
                let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y);
                let b = 2f32 * ((ray.direction.x * ray.origin.x) + (ray.direction.y * ray.origin.y));
                let c = (ray.origin.x * ray.origin.x) + (ray.origin.y * ray.origin.y) - (r * r);
                match quadratic(a, b, c) {
                    None => { },
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        let z2 = ray.at_time(t2).z;
                        if t1 >= 0f32 && z1 >= -hh && z1 <= hh { res.push(t1); };
                        if t2 >= 0f32 && z2 >= -hh && z2 <= hh { res.push(t2); };
                    },
                }

            },
            &Disc(_, r) => {
                if ray.direction.z > 0.0001 {
                    let t = -ray.origin.z / ray.direction.z;
                    let d = ray.at_time(t).distance_squared(&Point::origin());
                    if t >= 0f32 && d <= (r*r) { res.push(t); };
                }
            },
            &Plane(_, hw, hd) => {
                if ray.direction.z > 0.0001 {
                    let t = -ray.origin.z / ray.direction.z;
                    let p = ray.at_time(t);
                    if t >= 0f32 && p.x.abs() <= hw && p.y.abs() <= hd { res.push(t); };
                }
            },
            &Sphere(_, r) => {
                let a = ray.direction.magnitude_squared();
                let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
                let c = ray.origin.distance_squared(&Point::origin()) - (r * r);
                match quadratic(a, b, c) {
                    None => { },
                    Some((t1, t2)) => {
                        if t1 >= 0f32 { res.push(t1); };
                        if t2 >= 0f32 { res.push(t2); };
                    },
                }
            },
            &Triangle(_, v0, v1, v2) => {
                let e1 = v1.sub_p(&v0);
                let e2 = v2.sub_p(&v0);
                let h = ray.direction.cross(&e2);
                let a = e1.dot(&h);
                if a != 0f32 {
                    let f = 1f32 / a;
                    let s = ray.origin.sub_p(&v0);
                    let u = f * s.dot(&h);
                    if u >= 0f32 && u <= 1f32 {
                        let q = s.cross(&e1);
                        let v = f * ray.direction.dot(&q);
                        if v >= 0f32 && (u + v) <= 1f32 {
                            let t = f * e2.dot(&q);
                            if t >= 0f32 { res.push(t); };
                        }
                    }
                }
            },
            &Paraboloid(_, r, h) => {
                let a = (h * ray.direction.x * ray.direction.x + h * ray.direction.y * ray.direction.y) / (r * r);
                let b = (2f32 * h * ray.origin.x * ray.direction.x + 2f32 * h * ray.origin.y * ray.direction.y) / (r * r) - ray.direction.z;
                let c = (h * ray.origin.x * ray.origin.x + h * ray.origin.y * ray.origin.y) / (r * r) - ray.origin.z;
                match quadratic(a, b, c) {
                    None => { },
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        let z2 = ray.at_time(t2).z;
                        if t1 >= 0f32 && z1 >= 0f32 && z1 <= h { res.push(t1); };
                        if t2 >= 0f32 && z2 >= 0f32 && z2 <= h { res.push(t2); };
                    },
                }
            },
        };

        res
    }

    pub fn surface_area(&self) -> f32 {
        match self {
            &Box(_, hw, hh, hd) => (8f32 * hd * hw) + (8f32 * hd * hh) + (8f32 * hw * hh),
            &Cone(_, r, h) => r * (r * r + h * h).sqrt() * Float::pi(),
            &Cylinder(_, r, h) => 2f32 * r * h * Float::pi(),
            &Disc(_, r) => 2f32 * r * r * Float::pi(),
            &Plane(_, hw, hd) => 4f32 * hw * hd,
            &Sphere(_, r) => 4f32 * r * r * Float::pi(),
            &Triangle(_, a, b, c) => 0.5f32 * b.sub_p(&a).cross(&c.sub_p(&a)).magnitude(),
            &Paraboloid(_, r, h) => (r / (h * h)) * ((r * r + 4f32 * h * h) * 1.5f32 - r * r * r) * Float::frac_pi_6(),
        }
    }

    pub fn world_bound(&self) -> AABB {
        self.bound().transform(self.get_transform())
    }

    pub fn intersect(&self, r : &Ray) -> Option<f32> {
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

                    if t0 >= 0f32 { return Some(t0); }
                    if t1 >= 0f32 { return Some(t1); }
                }

                None
            },
            &Cone(_, r, h) => {
                let a = (h * h * ray.direction.x * ray.direction.x + h * h * ray.direction.y * ray.direction.y) / (r * r) + (-ray.direction.z * ray.direction.z);
                let b = (2f32 * h * h * ray.origin.x * ray.direction.x + 2f32 * h * h * ray.origin.y * ray.direction.y) / (r * r) + (-2f32 * ray.origin.z * ray.direction.z + 2f32 * ray.direction.z * h);
                let c = (h * h * ray.origin.x * ray.origin.x + h * h * ray.origin.y * ray.origin.y) / (r * r) + (-ray.origin.z * ray.origin.z + 2f32 * ray.origin.z * h - h * h);
                match quadratic(a, b, c) {
                    None => { None }
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        if t1 >= 0f32 && z1 >= 0f32 && z1 <= h { return Some(t1); }
                        let z2 = ray.at_time(t2).z;
                        if t2 >= 0f32 && z2 >= 0f32 && z2 <= h { return Some(t2); }
                        None
                    }
                }
            },
            &Cylinder(_, r, hh) => {
                let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y);
                let b = 2f32 * ((ray.direction.x * ray.origin.x) + (ray.direction.y * ray.origin.y));
                let c = (ray.origin.x * ray.origin.x) + (ray.origin.y * ray.origin.y) - (r * r);
                match quadratic(a, b, c) {
                    None => { None },
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        if t1 >= 0f32 && z1 >= -hh && z1 <= hh { return Some(t1); }
                        let z2 = ray.at_time(t2).z;
                        if t2 >= 0f32 && z2 >= -hh && z2 <= hh { return Some(t2); }
                        None
                    },
                }
            },
            &Disc(_, r) => {
                if ray.direction.z.abs() < 0.0001 { return None; }
                let t = -ray.origin.z / ray.direction.z;
                let d = ray.at_time(t).distance_squared(&Point::origin());
                if t >= 0f32 && d <= (r*r) { 
                    Some(t)
                } else {
                    None
                }
            },
            &Plane(_, hw, hd) => {
                if ray.direction.z.abs() < 0.0001 { return None; }
                let t = -ray.origin.z / ray.direction.z;
                let p = ray.at_time(t);
                if t >= 0f32 && p.x.abs() <= hw && p.y.abs() <= hd {
                    Some(t)
                } else { 
                    None
                }
            },
            &Sphere(_, r) => {
                let a = ray.direction.magnitude_squared();
                let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
                let c = ray.origin.distance_squared(&Point::origin()) - (r * r);
                match quadratic(a, b, c) {
                    None => { None },
                    Some((t1, t2)) => {
                        if t1 >= 0f32 { return Some(t1); }
                        if t2 >= 0f32 { return Some(t2); }
                        None
                    },
                }
            },
            &Triangle(_, v0, v1, v2) => {
                let e1 = v1.sub_p(&v0);
                let e2 = v2.sub_p(&v0);
                let h = ray.direction.cross(&e2);
                let a = e1.dot(&h);
                if a == 0f32 { return None; }
                let f = 1f32 / a;
                let s = ray.origin.sub_p(&v0);
                let u = f * s.dot(&h);
                if u < 0f32 || u > 1f32 { return None; }
                let q = s.cross(&e1);
                let v = f * ray.direction.dot(&q);
                if v < 0f32 || (u + v) > 1f32 { return None; }
                let t = f * e2.dot(&q);
                Some(t)
            },
            &Paraboloid(_, r, h) => {
                let a = (h * ray.direction.x * ray.direction.x + h * ray.direction.y * ray.direction.y) / (r * r);
                let b = (2f32 * h * ray.origin.x * ray.direction.x + 2f32 * h * ray.origin.y * ray.direction.y) / (r * r) - ray.direction.z;
                let c = (h * ray.origin.x * ray.origin.x + h * ray.origin.y * ray.origin.y) / (r * r) - ray.origin.z;
                match quadratic(a, b, c) {
                    None => { None },
                    Some((t1, t2)) => {
                        let z1 = ray.at_time(t1).z;
                        if t1 >= 0f32 && z1 >= 0f32 && z1 <= h { return Some(t1); }
                        let z2 = ray.at_time(t2).z;
                        if t2 >= 0f32 && z2 >= 0f32 && z2 <= h { return Some(t2); }
                        None
                    },
                }
            },
        }
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        match self.intersect(r) {
            None => false,
            Some(_) => true,
        }
    }
}

impl Trans for Shape {
    fn transform(&self, t : &Transform) -> Shape {
        match self {
            &Box(c, hw, hh, hd) => Box(t.compose(&c), hw, hh, hd),
            &Cone(c, r, h) => Cone(t.compose(&c), r, h), 
            &Cylinder(c, r, hh) => Cylinder(t.compose(&c), r, hh),
            &Disc(c, r) => Disc(t.compose(&c), r),
            &Plane(c, hw, hd) => Plane(t.compose(&c), hw, hd),
            &Sphere(c, r) => Sphere(t.compose(&c), r),
            &Triangle(c, a, b, d) => Triangle(t.compose(&c), a, b, d),
            &Paraboloid(c, r, h) => Paraboloid(t.compose(&c), r, h),
        }
    }
}

impl TransMut for Shape {
    fn transform_self(&mut self, t : &Transform) {
        match self {
            &Box(ref mut c, _, _, _) => { *c = t.compose(c); },
            &Cone(ref mut c, _, _) => { *c = t.compose(c); },
            &Cylinder(ref mut c, _, _) => { *c = t.compose(c); },
            &Disc(ref mut c, _) => { *c = t.compose(c); },
            &Plane(ref mut c, _, _) => { *c = t.compose(c); },
            &Sphere(ref mut c, _) => { *c = t.compose(c); },
            &Triangle(ref mut c, _, _, _) => { *c = t.compose(c); },
            &Paraboloid(ref mut c, _, _) => { *c = t.compose(c); },
        };
    }
}

#[test]
fn test_cylinder() {
    let c = Shape::new_unit_cylinder().rotate3(Float::frac_pi_2(), 0f32, 0f32).translate(&Vector::new(0f32, 0f32, 10f32));
    let r = Ray::z_axis();

    match c.intersect(&r) {
        None => assert!(false),
        Some(t) => assert_eq!(t, 9.5f32),
    }
}
