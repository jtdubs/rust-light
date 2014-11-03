use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;
use camera::Camera;

pub struct PerspectiveCamera {
    t : Transform,
    pub fov_y : f32,
}

impl PerspectiveCamera {
    pub fn new(fov_y : f32) -> PerspectiveCamera {
        PerspectiveCamera { t: Transform::identity(), fov_y: fov_y }
    }
}

impl Camera for PerspectiveCamera {
    fn cast(&self, film : &Film, fx : f32, fy : f32) -> Ray {
        let fw = film.width as f32;
        let fh = film.height as f32;
        let x = (fx / fw) * 2f32 - 1f32;
        let y = (fy / fh) * 2f32 - 1f32;
        let sx = (self.fov_y / 2f32).tan() * (fw / fh);
        let sy = (self.fov_y / 2f32).tan();
        let d = Vector::new(x * sx, y * sy, 1f32).normalize();
        let o = Point::origin();
        Ray::new(&o, &d).transform(&self.t.inverse())
    }
}

impl TransMut for PerspectiveCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
