use std::default::Default;
use std::f32::consts::*;

use crate::geometry::transform::{Transform,Trans,TransMut,HasTransform};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::point::Point;
use crate::geometry::vector::Vector;
use crate::math::quadratic;
use crate::shapes::shape::{Shape,ShapeIntersection};
use crate::shapes::surface_context::SurfaceContext;

#[derive(Copy, Clone)]
pub struct Paraboloid {
    transform   : Transform,
    radius      : f32,
    height      : f32,
    z_min       : f32,
    z_max       : f32,
    phi_max     : f32,
}

impl Paraboloid {
    pub fn new(radius : f32, height : f32) -> Paraboloid {
        Paraboloid { transform: Transform::identity(), radius: radius, height: height, z_min: 0f32, z_max: height, phi_max: 2f32 * PI }
    }

    pub fn unit() -> Paraboloid {
        Paraboloid::new(0.5f32, 1f32)
    }

    pub fn new_partial(radius : f32, height : f32, z_min : f32, z_max : f32, phi_max : f32) -> Paraboloid {
        Paraboloid { transform: Transform::identity(), radius: radius, height: height, z_min: z_min, z_max: z_max, phi_max: phi_max }
    }
}

impl Default for Paraboloid {
    fn default() -> Paraboloid {
        Paraboloid::unit()
    }
}

impl HasTransform for Paraboloid {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Shape for Paraboloid {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.radius, -self.radius, 0f32), Point::new(self.radius, self.radius, self.height)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn surface_area(&self) -> f32 {
        0f32 // TODO
        // (self.r / (self.h * self.h)) * ((self.r * self.r + 4f32 * self.h * self.h) * 1.5f32 - self.r * self.r * self.r) * core::f32::consts::FRAC_PI_6
    }

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection> {
        let ray = r.to(self);


        let m = self.height / (self.radius * self.radius);

        let a = m * (ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y);
        let b = 2f32 * m * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y) - ray.direction.z;
        let c = m * (ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y) - ray.origin.z;

        match quadratic(a, b, c) {
            None => { None }
            Some((t0, t1)) => {
                let mut thit = if t0 >= 0f32 { t0 } else { t1 };
                if thit < 0f32 {
                    return None
                }

                let mut phit = ray.at_time(thit);
                if phit.x == 0f32 && phit.y == 0f32 {
                    phit.x = 1e-5f32 * self.radius;
                }

                let mut phi = phit.y.atan2(phit.x);
                if phi < 0f32 {
                    phi += 2f32 * PI;
                }

                if phit.z < self.z_min || phit.z > self.z_max || phi > self.phi_max {
                    if thit == t1 {
                        return None
                    }

                    thit = t1;

                    phit = ray.at_time(thit);
                    if phit.x == 0f32 && phit.y == 0f32 {
                        phit.x = 1e-5f32 * self.radius;
                    }

                    phi = phit.y.atan2(phit.x);
                    if phi < 0f32 {
                        phi += 2f32 * PI;
                    }

                    if phit.z < self.z_min || phit.z > self.z_max || phi > self.phi_max {
                        return None
                    }
                }

                let u = phi / self.phi_max;
                let v = (phit.z - self.z_min) / (self.z_max - self.z_min);

                // TODO: reverse these
                let dpdv = Vector::new(-self.phi_max * phit.y, self.phi_max * phit.x, 0f32);
                let dpdu = (self.z_max - self.z_min) * Vector::new(phit.x / (2f32 * phit.z), phit.y / (2f32 * phit.z), 1f32);

                let d2pduu = -self.phi_max * self.phi_max * Vector::new(phit.x, phit.y, 0f32);
                let d2pduv = self.phi_max * (self.z_max - self.z_min) * Vector::new(-phit.y / (2f32 * phit.z), phit.x / (2f32 * phit.z), 0f32);
                let d2pdvv = -(self.z_max - self.z_min) * (self.z_max - self.z_min) * Vector::new(phit.x / (4f32 * phit.z * phit.z), phit.y / (4f32 * phit.z * phit.z), 0f32);

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

                return Some(ShapeIntersection::new(*r, thit, SurfaceContext::new(phit, (u, v), (dpdu, dpdv), (dndu, dndv))));
            }
        }
    }
}

impl Trans for Paraboloid {
    type Output=Paraboloid;

    fn transform(&self, t : &Transform) -> Paraboloid {
        Paraboloid { transform: *t + self.transform, .. *self }
    }
}

impl TransMut for Paraboloid {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}

