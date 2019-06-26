use std::default::Default;

use crate::geometry::{Transform, Trans, TransMut, HasTransform, BoundingBox, Ray, Point, Vector, Normal};
use crate::shapes::{Shape, ShapeIntersection, SurfaceContext};

#[derive(Copy, Clone)]
pub struct Plane {
    transform : Transform,
    dx : f32,
    dy : f32,
}

impl Plane {
    pub fn new(dx : f32, dy : f32) -> Plane {
        Plane { transform: Transform::identity(), dx: dx, dy: dy }
    }

    pub fn unit() -> Plane {
        Plane::new(0.5f32, 0.5f32)
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane::unit()
    }
}

impl HasTransform for Plane {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Shape for Plane {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.dx, -self.dy, 0f32), Point::new(self.dx, self.dy, 0f32)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.dx * self.dy
    }

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection> {
        let ray = r.to(self);

        if ray.direction.z.abs() < 1e-7f32 { return None; }

        let thit = -ray.origin.z / ray.direction.z;
        if thit < 0f32 {
            return None;
        }

        let phit = ray.at_time(thit);

        if phit.x.abs() > self.dx || phit.y.abs() > self.dy {
            return None;
        }

        let u = (phit.x + self.dx) / (self.dx * 2f32);
        let v = (phit.y + self.dy) / (self.dy * 2f32);

        let dpdu = Vector::unit_x();
        let dpdv = Vector::unit_y();

        let normal = Normal::new(0f32, 0f32, 1f32).face_forward(&ray.direction);

        let dndu = Normal::new(0f32, 0f32, 0f32);
        let dndv = Normal::new(0f32, 0f32, 0f32);

        return Some(ShapeIntersection::new(*r, thit, SurfaceContext::new(phit, normal, (u, v), (dpdu, dpdv), (dndu, dndv))));
    }
}

impl Trans for Plane {
    type Output=Plane;

    fn transform(&self, t : &Transform) -> Plane {
        Plane { transform: *t + self.transform, .. *self }
    }
}

impl TransMut for Plane {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
