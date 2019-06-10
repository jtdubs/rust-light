use std::rc::{Rc};

use crate::geometry::ray::Ray;
use crate::geometry::bounding_box::BoundingBox;
use crate::shapes::shape::Intersection;
use crate::shapes::shape::Shape;

pub struct Scene {
    pub primitives : Vec<(BoundingBox, Rc<dyn Shape>)>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { primitives: Vec::new() }
    }

    pub fn add<T : Shape + 'static>(&mut self, p : T) {
        self.primitives.push((p.world_bound(), Rc::new(p)));
    }

    pub fn bounds(&self) -> BoundingBox {
        let mut bounds = BoundingBox::new();
        for &(a, _) in self.primitives.iter() {
            bounds.add_self_bounding_box(&a);
        }
        bounds
    }

    pub fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let mut first_intersection = None;
        for &(ref a, ref p) in self.primitives.iter() {
            if a.intersects(r) {
                match p.intersect(r) {
                    None => { }
                    Some(ref i) => {
                        match first_intersection {
                            None => first_intersection = Some(*i),
                            Some(i0) => if i.time < i0.time { first_intersection = Some(*i) },
                        }
                    }
                }
            }
        }

        first_intersection
    }
}
