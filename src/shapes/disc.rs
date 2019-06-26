use std::f32::consts::*;
use std::default::Default;

use crate::geometry::{Transform, Trans, TransMut, HasTransform, BoundingBox, Ray, Point, Vector, Normal};
use crate::shapes::{Shape, ShapeIntersection, SurfaceContext};

#[derive(Copy, Clone)]
pub struct Disc {
    transform    : Transform,
    inner_radius : f32,
    outer_radius : f32,
    phi_max      : f32,
}

impl Disc {
    pub fn new(radius : f32) -> Disc {
        Disc {
            transform:    Transform::identity(),
            inner_radius: 0f32,
            outer_radius: radius,
            phi_max:      2f32 * PI,
        }
    }

    pub fn new_annulus(inner_radius : f32, outer_radius : f32) -> Disc {
        Disc {
            transform:    Transform::identity(),
            inner_radius: inner_radius,
            outer_radius: outer_radius,
            phi_max:      2f32 * PI,
        }
    }

    pub fn new_partial(radius : f32, phi_max : f32) -> Disc {
        Disc {
            transform:    Transform::identity(),
            inner_radius: 0f32,
            outer_radius: radius,
            phi_max:      phi_max,
        }
    }

    pub fn new_partial_annulus(inner_radius : f32, outer_radius : f32, phi_max : f32) -> Disc {
        Disc {
            transform:    Transform::identity(),
            inner_radius: inner_radius,
            outer_radius: outer_radius,
            phi_max:      phi_max,
        }
    }

    pub fn unit() -> Disc {
        Disc::new(1f32)
    }
}

impl Default for Disc {
    fn default() -> Disc {
        Disc::unit()
    }
}

impl HasTransform for Disc {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Shape for Disc {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.outer_radius, -self.outer_radius, 0f32), Point::new(self.outer_radius, self.outer_radius, 0f32)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn surface_area(&self) -> f32 {
        ((self.outer_radius * self.outer_radius) - (self.inner_radius * self.inner_radius)) * (self.phi_max / 2f32)
    }

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection> {
        let ray = r.to(self);

        if ray.direction.z.abs() < 1e-7f32 { return None; }

        let thit = -ray.origin.z / ray.direction.z;
        if thit < 0f32 {
            return None;
        }

        let phit = ray.at_time(thit);

        let dist2 = phit.x * phit.x + phit.y * phit.y;
        if dist2 > (self.outer_radius * self.outer_radius) || dist2 < (self.inner_radius * self.inner_radius) {
            return None;
        }

        let mut phi = phit.y.atan2(phit.x);
        if phi < 0f32 {
            phi += 2f32 * PI;
        }
        if phi > self.phi_max {
            return None;
        }

        let u = phi / self.phi_max;
        let v = 1f32 - ((dist2.sqrt() - self.inner_radius) / (self.outer_radius - self.inner_radius));

        let dpdu = (self.phi_max / FRAC_PI_2) * Vector::new(-self.phi_max * phit.y, self.phi_max * phit.x, 0f32);
        let dpdv = ((self.outer_radius - self.inner_radius) / self.outer_radius) * Vector::new(-phit.x / (1f32-v), -phit.y / (1f32-v), 0f32);

        let normal = dpdu.cross(&dpdv).normalize().to_normal().face_forward(&ray.direction);

        let dndu = Normal::new(0f32, 0f32, 0f32);
        let dndv = Normal::new(0f32, 0f32, 0f32);

        return Some(ShapeIntersection::new(*r, thit, SurfaceContext::new(phit, normal, (u, v), (dpdu, dpdv), (dndu, dndv))));
    }
}

impl Trans for Disc {
    type Output=Disc;

    fn transform(&self, t : &Transform) -> Disc {
        Disc { transform: *t + self.transform, .. *self }
    }
}

impl TransMut for Disc {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}

