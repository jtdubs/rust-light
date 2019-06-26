use log::*;
use std::default::Default;
use std::f32::consts::PI;

use crate::geometry::{Transform, Trans, TransMut, HasTransform, BoundingBox, Ray, Point, Vector};
use crate::math::quadratic;
use crate::shapes::{Shape, ShapeIntersection, SurfaceContext};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    transform : Transform,
    radius    : f32,
    z_min     : f32,
    z_max     : f32,
    theta_min : f32,
    theta_max : f32,
    phi_max   : f32,
}

impl Sphere {
    pub fn new(radius : f32) -> Sphere {
        Sphere {
            transform: Transform::identity(),
		    radius:    radius,
		    z_min:    -radius,
		    z_max:     radius,
		    theta_min: 0f32,
		    theta_max: PI,
		    phi_max:   2f32 * PI
        }
    }

    pub fn new_partial(radius : f32, (z_min, z_max) : (f32, f32), phi_max : f32) -> Sphere {
        let z_min = z_min.max(-radius).min(radius);
        let z_max = z_max.max(-radius).min(radius);
        let phi_max = phi_max.max(0f32).min(2f32 * PI);

        Sphere {
            transform: Transform::identity(),
		    radius:    radius,
		    z_min:     z_min,
		    z_max:     z_max,
		    theta_min: (z_min / radius).acos(),
		    theta_max: (z_max / radius).acos(),
		    phi_max:   phi_max
        }
    }

    pub fn unit() -> Sphere {
        Sphere::new(0.5f32)
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere::unit()
    }
}

impl HasTransform for Sphere {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Shape for Sphere {
    fn bound(&self) -> BoundingBox {
        BoundingBox::for_points(&[Point::new(-self.radius, -self.radius, self.z_min), Point::new(self.radius, self.radius, self.z_max)])
    }

    fn surface_area(&self) -> f32 {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }

    fn world_bound(&self) -> BoundingBox {
        self.bound().from(self)
    }

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection> {
        let ray = r.to(self);

        let a = ray.direction.magnitude_squared();
        let b = 2f32 * ray.direction.dot(&ray.origin.sub_p(&Point::origin()));
        let c = ray.origin.distance_squared(&Point::origin()) - (self.radius * self.radius);

        match quadratic(a, b, c) {
            None => { None },
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

                debug!("sphere.phi  = {:?}", phi);

                if (self.z_min > -self.radius && phit.z < self.z_min) || (self.z_max < self.radius && phit.z > self.z_max) || phi > self.phi_max {
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

                    if (self.z_min > -self.radius && phit.z < self.z_min) || (self.z_max < self.radius && phit.z > self.z_max) || phi > self.phi_max {
                        return None
                    }
                }

                let u = phi / self.phi_max;
                let theta = (phit.z / self.radius).min(1f32).max(-1f32).acos();
                let v = (theta - self.theta_min) / (self.theta_max - self.theta_min);

                let zr = (phit.x*phit.x + phit.y*phit.y).sqrt();
                let cosphi = phit.x / zr;
                let sinphi = phit.y / zr;

                let dpdu = Vector::new(-self.phi_max * phit.y, self.phi_max * phit.x, 0f32);
                let dpdv = (self.theta_max - self.theta_min) * Vector::new(phit.z * cosphi, phit.z * sinphi, -self.radius * theta.sin());

                let normal = dpdu.cross(&dpdv).normalize().to_normal().face_forward(&ray.direction);

                let d2pduu = -self.phi_max * self.phi_max * Vector::new(phit.x, phit.y, 0f32);
                let d2pduv = (self.theta_max - self.theta_min) * phit.z * self.phi_max * Vector::new(-sinphi, cosphi, 0f32);
                let d2pdvv = -(self.theta_max - self.theta_min) * (self.theta_max * self.theta_min) * Vector::new(phit.x, phit.y, phit.z);

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
            },
        }
    }
}

impl Trans for Sphere {
    type Output=Sphere;

    fn transform(&self, t : &Transform) -> Sphere {
        Sphere { transform: *t + self.transform, .. *self }
    }
}

impl TransMut for Sphere {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
