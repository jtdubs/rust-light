use geometry::transform::{Transform,Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;
use geometry::point::Point;
use math::quadratic;
use shapes::shape::Shape;

pub struct Sphere {
    t : Transform,
    r : f32
}

impl Sphere {
    pub fn new(diameter : f32) -> Sphere {
        Sphere { t: Transform::identity(), r: diameter / 2f32 }
    }

    pub fn unit() -> Sphere {
        Sphere::new(1f32)
    }
}

impl Shape for Sphere {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points([Point::new(-self.r, -self.r, -self.r), Point::new(self.r, self.r, self.r)])
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.r * self.r * Float::pi()
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().transform(&self.t)
    }

    fn intersections(&self, r : &Ray) -> Vec<f32> {
        let mut res = Vec::new();
        let ray = r.transform(&self.t.inverse());

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { },
            Some((t1, t2)) => {
                if t1 >= 0f32 { res.push(t1); };
                if t2 >= 0f32 { res.push(t2); };
            },
        }

        res
    }

    fn intersect(&self, r : &Ray) -> Option<f32> {
        let ray = r.transform(&self.t.inverse());

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);
        match quadratic(a, b, c) {
            None => { None },
            Some((t1, t2)) => {
                if t1 >= 0f32 { return Some(t1); }
                if t2 >= 0f32 { return Some(t2); }
                None
            },
        }
    }
}

impl Trans for Sphere {
    fn transform(&self, t : &Transform) -> Sphere {
        Sphere { t: t.compose(&self.t), r: self.r }
    }
}

impl TransMut for Sphere {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
