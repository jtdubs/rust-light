use primitive::Primitive;
use geometry::ray::Ray;
use geometry::bounding_box::BoundingBox;

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

    pub fn intersect(&self, r : &Ray) -> Option<f32> {
        let mut first_intersection = None;

        for &(ref a, ref p) in self.primitives.iter() {
            if a.intersects(r) {
                match p.intersect(r) {
                    None => { }
                    Some(t) => {
                        match first_intersection {
                            None => first_intersection = Some(t),
                            Some(t0) => if t < t0 { first_intersection = Some(t) },
                        }
                    }
                }
            }
        }

        first_intersection
    }
}
