use camera::Camera;
use primitive::Primitive;
use ray::Ray;

pub struct Scene {
    pub camera : Camera,
    pub primitives : Vec<Primitive>,
}

impl Scene {
    pub fn new(c : Camera) -> Scene {
        Scene { camera: c, primitives: Vec::new() }
    }

    pub fn add(&mut self, p : Primitive) {
        self.primitives.push(p);
    }

    pub fn intersect(&self, r : &Ray) -> Option<f64> {
        let mut first_intersection = None;

        for p in self.primitives.iter() {
            match p.intersect(r) {
                None => { },
                Some(t) => {
                    match first_intersection {
                        None => first_intersection = Some(t),
                        Some(t0) => if t < t0 { first_intersection = Some(t) },
                    }
                }
            }
        }

        first_intersection
    }
}
