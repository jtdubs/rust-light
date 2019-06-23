use log::*;
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::shapes::surface_context::SurfaceContext;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub ray : Ray,
    pub time : f32,
    pub context : SurfaceContext,
}

impl Intersection {
    pub fn new(ray : Ray, time : f32, context : SurfaceContext) -> Intersection {
        info!("intersection.ray    = {:?}", ray);
        info!("intersection.time   = {:?}", time);
        info!("intersection.point  = {:?}", context.p);
        info!("intersection.normal = {:?}", context.n);
        info!("intersection.uv     = {:?}", (context.u, context.v));
        info!("intersection.dpdu   = {:?}", context.dpdu);
        info!("intersection.dpdv   = {:?}", context.dpdv);
        info!("intersection.dndu   = {:?}", context.dndu);
        info!("intersection.dndv   = {:?}", context.dndv);

        Intersection { ray: ray, time: time, context: context }
    }
}

pub trait Shape : Send + Sync {
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
