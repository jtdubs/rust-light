use log::*;
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::geometry::transform::HasTransform;
use crate::shapes::surface_context::SurfaceContext;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub ray : Ray,
    pub time : f32,
    pub context : SurfaceContext,
}

impl Intersection {
    pub fn new(ray : Ray, time : f32, context : SurfaceContext) -> Intersection {
        debug!("intersection.ray    = {:?}", ray);
        debug!("intersection.time   = {:?}", time);
        debug!("intersection.point  = {:?}", context.p);
        debug!("intersection.normal = {:?}", context.n);
        debug!("intersection.uv     = {:?}", (context.u, context.v));
        debug!("intersection.dpdu   = {:?}", context.dpdu);
        debug!("intersection.dpdv   = {:?}", context.dpdv);
        debug!("intersection.dndu   = {:?}", context.dndu);
        debug!("intersection.dndv   = {:?}", context.dndv);

        Intersection { ray: ray, time: time, context: context }
    }
}

pub trait Shape : HasTransform + Send + Sync {
    fn bound(&self) -> BoundingBox;
    fn world_bound(&self) -> BoundingBox;

    fn surface_area(&self) -> f32;

    fn intersect(&self, r : &Ray) -> Option<Intersection>;

    fn intersects(&self, r : &Ray) -> bool {
        match self.intersect(r) {
            None => false,
            Some(_) => true,
        }
    }
}
