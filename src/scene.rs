use std::sync::Arc;

use crate::geometry::ray::Ray;
use crate::geometry::bounding_box::BoundingBox;
use crate::shapes::shape::{Shape,ShapeIntersection};
use crate::shapes::surface_context::SurfaceContext;
use crate::geometry::transform::{Transform,HasTransform};

pub struct Scene {
    pub primitives : Vec<(BoundingBox, Arc<dyn Shape>)>,
    pub bounds : BoundingBox
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: Vec::new(),
            bounds: BoundingBox::empty()
        }
    }

    pub fn add(&mut self, p : Arc<dyn Shape>) {
        let b = p.world_bound();
        self.primitives.push((b, p));
        self.bounds.add_self_bounding_box(&b);
    }

    pub fn intersect(&self, r : &Ray) -> Option<SceneIntersection> {
        let mut first_intersection = None;

        if self.bounds.intersects(&r) {
            for (a, p) in self.primitives.iter() {
                if a.intersects(r) {
                    match p.intersect(r) {
                        None => { }
                        Some(i) => {
                            match first_intersection {
                                None => first_intersection = Some(SceneIntersection::new(p.clone(), i)),
                                Some(ref i0) => if i.time < i0.time { first_intersection = Some(SceneIntersection::new(p.clone(), i)) },
                            }
                        }
                    }
                }
            }
        }

        first_intersection
    }
}

impl HasTransform for Arc<dyn Shape> {
    fn get_transform(&self) -> &Transform {
        (**self).get_transform()
    }
}

#[derive(Clone)]
pub struct SceneIntersection {
    pub ray : Ray,
    pub time : f32,
    pub shape : Arc<dyn Shape>,
    pub context : SurfaceContext,
}

impl SceneIntersection {
    pub fn new(shape : Arc<dyn Shape>, i : ShapeIntersection) -> SceneIntersection {
        SceneIntersection {
            ray: i.ray,
            time: i.time,
            shape: shape,
            context: i.context
        }
    }
}
