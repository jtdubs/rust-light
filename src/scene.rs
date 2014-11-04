use primitive::Primitive;
use ray::Ray;
use aabb::AABB;

pub struct Scene<'a> {
    pub primitives : Vec<(AABB, Primitive<'a>)>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { primitives: Vec::new() }
    }

    pub fn add(&mut self, p : Primitive<'a>) {
        self.primitives.push((p.world_bound(), p));
    }

    pub fn bounds(&self) -> AABB {
        let mut bounds = AABB::new();
        for &(a, _) in self.primitives.iter() {
            bounds.add_self_aabb(&a);
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
