use log::*;
use std::default::Default;

use crate::geometry::transform::{Transform,HasTransform,Trans,TransMut};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::math::quadratic;
use crate::shapes::shape::{Shape,Intersection};
use crate::shapes::surface_context::SurfaceContext;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    t : Transform,
    r : f32
}

impl Sphere {
    pub fn new(diameter : f32) -> Sphere {
        Sphere { t: Transform::identity(), r: diameter / 2f32 }
    }

    // TODO: add new_partial

    pub fn unit() -> Sphere {
        Sphere::new(1f32)
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere::unit()
    }
}

impl HasTransform for Sphere {
    fn get_transform(&self) -> &Transform {
        &self.t
    }
}

impl Shape for Sphere {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.r, -self.r, -self.r), Point::new(self.r, self.r, self.r)])
    }

    fn surface_area(&self) -> f32 {
        4f32 * self.r * self.r * core::f32::consts::PI
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let ray = r.to(self);

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);

        match quadratic(a, b, c) {
            None => { None },
            Some((t1, t2)) => {
                let twopi = 2f32 * std::f32::consts::PI;
                let fourpi2 = twopi * twopi;

                let thit = if t1 >= 0f32 { t1 } else { t2 };
                if thit < 0f32 {
                    return None
                }

                let mut phit = ray.at_time(thit);
                if phit.x == 0f32 && phit.y == 0f32 {
                    phit.x = 1e-5f32 * self.r;
                }

                let mut phi = phit.y.atan2(phit.x);
                if phi < 0f32 {
                    phi += twopi;
                }

                debug!("sphere.phi  = {:?}", phi);

                let u = phi / twopi;
                let theta = (phit.z / self.r).min(1f32).max(-1f32).acos();
                let v = theta / std::f32::consts::PI;

                let zr = (phit.x*phit.x + phit.y*phit.y).sqrt();
                let cosphi = phit.x / zr;
                let sinphi = phit.y / zr;
                let dpdu = Vector::new(-twopi * phit.y, twopi * phit.x, 0f32);
                let dpdv = std::f32::consts::PI * Vector::new(phit.z * cosphi, phit.z * sinphi, -self.r * theta.sin());

                let d2pduu = -fourpi2 * Vector::new(phit.x, phit.y, 0f32);
                let d2pduv = fourpi2 * phit.z * Vector::new(-sinphi, cosphi, 0f32);
                let d2pdvv = -fourpi2 * Vector::new(phit.x, phit.y, phit.z);

                let c_e = dpdu.dot(&dpdu);
                let c_f = dpdu.dot(&dpdv);
                let c_g = dpdv.dot(&dpdv);
                let n = dpdu.cross(&dpdv).normalize();
                let e = n.dot(&d2pduu);
                let f = n.dot(&d2pduv);
                let g = n.dot(&d2pdvv);

                let egf2 = 1f32 / (c_e*c_g - c_f*c_f);

                let dndu = ((f*c_f - e*c_e) * egf2 * dpdu + (e*c_f - f*c_e) * egf2 * dpdv).to_normal();
                let dndv = ((g*c_f - f*c_e) * egf2 * dpdu + (f*c_f - g*c_e) * egf2 * dpdv).to_normal();

                return Some(Intersection::new(*r, thit, SurfaceContext::new(r.at_time(thit), (u, v), (dpdu, dpdv), (dndu, dndv))));
            },
        }
    }

    fn intersects(&self, r : &Ray) -> bool {
        let ray = r.to(self);

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.r * self.r);

        match quadratic(a, b, c) {
            None => false,
            Some((t1, t2)) => {
                t1 >= 0f32 || t2 >= 0f32
            },
        }
    }
}

impl Trans for Sphere {
    type Output=Sphere;

    fn transform(&self, t : &Transform) -> Sphere {
        Sphere { t: *t + self.t, .. *self }
    }
}

impl TransMut for Sphere {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
