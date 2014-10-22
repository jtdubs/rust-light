use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;

pub trait Camera<'a> : TransMut {
    fn get_film(&self) -> &'a Film;
    fn cast(&self, fx : f64, fy : f64) -> Ray;
}

pub struct OrthoCamera<'a> {
    t : Transform,
    f : &'a Film,
    s : f64,
}

impl<'a> OrthoCamera<'a> {
    pub fn new(f : &'a Film, s : f64) -> OrthoCamera<'a> {
        OrthoCamera { t: Transform::identity(), f: f, s: s }
    }
}

impl<'a> Camera<'a> for OrthoCamera<'a> {
    fn get_film(&self) -> &'a Film {
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

pub struct PerspectiveCamera<'a> {
    t : Transform,
    f : &'a Film,
    fov : f64
}

impl<'a> PerspectiveCamera<'a> {
    pub fn new(f : &'a Film, fov : f64) -> PerspectiveCamera<'a> {
        PerspectiveCamera { t: Transform::identity(), f: f, fov: fov }
    }

    pub fn get_fov_y(&self) -> f64 {
        self.fov
    }
}

impl<'a> Camera<'a> for PerspectiveCamera<'a> {
    fn get_film(&self) -> &'a Film {
        self.f
    }

    fn cast(&self, fx : f64, fy : f64) -> Ray {
        let fw = self.f.width as f64;
        let fh = self.f.height as f64;
        let x = (fx / fw) * 2f64 - 1f64;
        let y = (fy / fh) * 2f64 - 1f64;
        let sx = (self.fov / 2f64).tan() * (fw / fh);
        let sy = (self.fov / 2f64).tan();
        let d = Vector::new(x*sx, y*sy, 1f64).normalize();
        let o = Point::origin();
        Ray::new(&o, &d)
    }
}

impl<'a> TransMut for PerspectiveCamera<'a> {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
