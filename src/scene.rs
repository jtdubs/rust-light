use crate::geometry::ray::Ray;
use crate::geometry::bounding_box::BoundingBox;
use crate::shapes::shape::Intersection;
use crate::shapes::shape::Shape;

pub struct Scene {
    pub primitives : Vec<(BoundingBox, Box<dyn Shape>)>,
    pub bounds : BoundingBox
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: Vec::new(),
            bounds: BoundingBox::empty()
        }
    }

    pub fn add(&mut self, p : Box<dyn Shape>) {
        let b = p.world_bound();
        self.primitives.push((b, p));
        self.bounds.add_self_bounding_box(&b);
    }

    pub fn intersect(&self, r : &Ray) -> Option<Intersection> {
        let mut first_intersection = None;

        if self.bounds.intersects(&r) {
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
        }

        first_intersection
    }
}
