use primitive::Primitive;
use geometry::ray::Ray;
use geometry::bounding_box::BoundingBox;
use shapes::shape::Intersection;

pub struct Scene<'a> {
    pub primitives : Vec<(BoundingBox, Primitive<'a>)>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { primitives: Vec::new() }
    }

    pub fn add(&mut self, p : Primitive<'a>) {
        self.primitives.push((p.world_bound(), p));
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
