use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;
use camera::Camera;

pub struct OrthographicCamera {
    t : Transform,
    pub scale : f32,
}

impl OrthographicCamera {
    pub fn new(scale : f32) -> OrthographicCamera {
        OrthographicCamera { t: Transform::identity(), scale: scale }
    }
}

impl Camera for OrthographicCamera {
    fn cast(&self, film : &Film, fx : f32, fy : f32) -> Ray {
        let fw = film.width as f32;
        let fh = film.height as f32;
        let x = fx - (fw / 2f32);
        let y = fy - (fh / 2f32);
        let d = Vector::unit_z();
        let o = Point::new(x*self.scale, y*self.scale, 0f32);
        Ray::new(&o, &d).transform(&self.t.inverse())
    }
}

impl TransMut for OrthographicCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
