use crate::geometry::{BoundingBox, Ray, HasTransform};
use crate::shapes::SurfaceContext;

pub trait Shape : HasTransform + Send + Sync {
    fn bound(&self) -> BoundingBox;
    fn world_bound(&self) -> BoundingBox;

    fn surface_area(&self) -> f32;

    fn intersect(&self, r : &Ray) -> Option<ShapeIntersection>;

    fn intersects(&self, r : &Ray) -> bool {
        match self.intersect(r) {
            None => false,
            Some(_) => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ShapeIntersection {
    pub ray : Ray,
    pub time : f32,
    pub context : SurfaceContext,
}

impl ShapeIntersection {
    pub fn new(ray : Ray, time : f32, context : SurfaceContext) -> ShapeIntersection {
        ShapeIntersection { ray: ray, time: time, context: context }
    }
}

