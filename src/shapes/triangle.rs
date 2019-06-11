use std::default::Default;

use crate::geometry::transform::{Transform,Trans,TransMut};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::shapes::shape::{Shape,Intersection};

#[derive(Copy, Clone)]
pub struct Triangle {
    t : Transform,
    a : Point,
    b : Point,
    c : Point,
}

impl Triangle {
    pub fn new(a : &Point, b : &Point, c : &Point) -> Triangle {
        Triangle { t: Transform::identity(), a: a.clone(), b: b.clone(), c: c.clone() }
    }

    pub fn unit() -> Triangle {
        Triangle::new(
            &Point::origin().sub_v(&Vector::unit_x().div_s(2f32)).sub_v(&Vector::unit_y().div_s(2f32)),
            &Point::origin().add_v(&Vector::unit_x().div_s(2f32)).sub_v(&Vector::unit_y().div_s(2f32)),
            &Point::origin().add_v(&Vector::unit_y().div_s(2f32)))
    }
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle::unit()
    }
}

impl Shape for Triangle {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[self.a, self.b, self.c])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().transform(&self.t)
    }

    fn surface_area(&self) -> f32 {
        0.5f32 * self.b.sub_p(&self.a).cross(&self.c.sub_p(&self.a)).magnitude()
    }

    fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        let mut res = Vec::new();
        let ray = r.transform(&-self.t);

        let e1 = self.b.sub_p(&self.a);
        let e2 = self.c.sub_p(&self.a);
        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if a != 0f32 {
            let f = 1f32 / a;
            let s = ray.origin.sub_p(&self.a);
            let u = f * s.dot(&h);
            if u >= 0f32 && u <= 1f32 {
                let q = s.cross(&e1);
                let v = f * ray.direction.dot(&q);
                if v >= 0f32 && (u + v) <= 1f32 {
                    let t = f * e2.dot(&q);
                    if t >= 0f32 { 
                        res.push(Intersection::new(r, t, &r.at_time(t))); 
                    };
                }
            }
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r.transform(&-self.t);

        let e1 = self.b.sub_p(&self.a);
        let e2 = self.c.sub_p(&self.a);
        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if a == 0f32 { return None; }
        let f = 1f32 / a;
        let s = ray.origin.sub_p(&self.a);
        let u = f * s.dot(&h);
        if u < 0f32 || u > 1f32 { return None; }
        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0f32 || (u + v) > 1f32 { return None; }
        let t = f * e2.dot(&q);

        Some(Intersection::new(r, t, &r.at_time(t)))
    }
}

impl Trans for Triangle {
    type Output=Triangle;

    fn transform(&self, t : &Transform) -> Triangle {
        Triangle { t: *t + self.t, .. *self }
    }
}

impl TransMut for Triangle {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}

