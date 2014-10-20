use film::Film;
use geometry::transform::{Transform,Trans,TransMut};
use geometry::ray::Ray;
use geometry::vector::Vector;
use geometry::point::Point;

pub trait Camera<'a> : TransMut {
    fn get_film(&self) -> &'a Film<'a>;
    fn cast(&self, fx : f64, fy : f64) -> Ray;
}

pub struct OrthoCamera<'a> {
    t : Transform,
    f : &'a Film<'a>,
    s : f64,
}

impl<'a> OrthoCamera<'a> {
    pub fn new(f : &'a Film<'a>, s : f64) -> OrthoCamera<'a> {
        OrthoCamera { t: Transform::identity(), f: f, s: s }
    }
}

impl<'a> Camera<'a> for OrthoCamera<'a> {
    fn get_film(&self) -> &'a Film<'a> {
        self.f
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

impl<'a> TransMut for OrthoCamera<'a> {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
