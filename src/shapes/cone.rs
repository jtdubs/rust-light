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
pub struct Cone {
    transform : Transform,
    radius    : f32,
    height    : f32,
    z_min     : f32,
    z_max     : f32,
    phi_max   : f32,
}

impl Cone {
    pub fn new(radius : f32, height : f32) -> Cone {
        Cone { transform: Transform::identity(), radius: radius, height: height, z_min: 0f32, z_max: height, phi_max: 2f32 * PI }
    }

    pub fn new_partial(radius : f32, height: f32, z_min : f32, z_max : f32, phi_max : f32) -> Cone {
        Cone { transform: Transform::identity(), radius: radius, height: height, z_min: z_min, z_max: z_max, phi_max: phi_max }
    }

    pub fn unit() -> Cone {
        Cone::new(0.5f32, 1f32)
    }
}

impl Default for Cone {
    fn default() -> Cone {
        Cone::unit()
    }
}

impl HasTransform for Cone {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Shape for Cone {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.radius, -self.radius, 0f32), Point::new(self.radius, self.radius, self.height)])
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn surface_area(&self) -> f32 {
        ((self.phi_max * self.radius) / (2f32 * self.height)) * (self.z_max - self.z_min) * (self.radius + (self.height * self.height + self.radius * self.radius).sqrt())
    }

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection> {
        let ray = r.to(self);

        let m = (self.height * self.height) / (self.radius * self.radius);

        let a = m * (ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y) - (ray.direction.z * ray.direction.z);
        let b = 2f32 * (m * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y) + (-ray.origin.z * ray.direction.z + ray.direction.z * self.height));
        let c = m * (ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y) + (-ray.origin.z * ray.origin.z + 2f32 * ray.origin.z * self.height - self.height * self.height);

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

                let dpdu = Vector::new(-self.phi_max * phit.y, self.phi_max * phit.x, 0f32);
                let dpdv = Vector::new(-phit.x / (1f32 - v), phit.y / (1f32 - v), self.z_max - self.z_min);

                let normal = dpdu.cross(&dpdv).normalize().to_normal().face_forward(&ray.direction);

                let d2pduu = -self.phi_max * self.phi_max * Vector::new(phit.x, phit.y, 0f32);
                let d2pduv = (self.phi_max / (1f32 - v)) * Vector::new(phit.y, -phit.x, 0f32);
                let d2pdvv = Vector::zero();

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

                return Some(ShapeIntersection::new(*r, thit, SurfaceContext::new(phit, normal, (u, v), (dpdu, dpdv), (dndu, dndv))));
            }
        }
    }
}

impl Trans for Cone {
    type Output=Cone;

    fn transform(&self, t : &Transform) -> Cone {
        Cone { transform: *t + self.transform, .. *self }
    }
}

impl TransMut for Cone {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
