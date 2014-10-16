use film::Film;
use filter::Filter;
use geometry::transform::{Transform,Trans,TransMut};
use geometry::ray::Ray;
use geometry::vector::Vector;
use geometry::point::Point;

pub trait Camera : TransMut {
    fn get_film(&self) -> &Film;
    fn cast(&self, fx : f64, fy : f64) -> Ray;
}

pub struct OrthoCamera {
    t : Transform,
    f : &Film,
    s : f64,
}

impl OrthoCamera {
    pub fn new(f : &Film, s : f64) -> OrthoCamera {
        OrthoCamera { t: Transform::identity(), f: Film::new_1080(f), s: s }
    }
}

impl Camera for OrthoCamera {
    fn get_film(&self) -> &Film {
        &self.f
    }

    fn cast(&self, fx : f64, fy : f64) -> Ray {
        let fw = self.f.width as f64;
        let fh = self.f.height as f64;
        let x = fx - (fw / 2f64);
        let y = fy - (fh / 2f64);
        let d = Vector::unit_z();
        let o = Point::new(x*self.s, y*self.s, 0f64);
        Ray::new(&o, &d).transform(&self.t.inverse())
    }
}

impl TransMut for OrthoCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
