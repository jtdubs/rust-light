use camera::Camera;
use primitive::Primitive;

pub struct Scene {
    c : Camera,
    ps : Vec<Primitive>,
}

impl Scene {
    pub fn new(c : Camera) -> Scene {
        Scene { c: c, ps: Vec::new() }
    }

    pub fn add(&mut self, p : Primitive) {
        self.ps.push(p);
    }

    pub fn get_camera(&self) -> &Camera {
        &self.c
    }
}
