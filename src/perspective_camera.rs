use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;
use camera::Camera;

pub struct PerspectiveCamera {
    t : Transform,
    pub fov_y : f32,
    fov_x_tan : f32,
    fov_y_tan : f32,
}

impl PerspectiveCamera {
    pub fn new(fov_y : f32, aspect_ratio : f32) -> PerspectiveCamera {
        let fov_y_tan = (fov_y / 2f32).tan();
        let fov_x_tan = fov_y_tan * aspect_ratio;
        PerspectiveCamera { t: Transform::identity(), fov_y: fov_y, fov_x_tan: fov_x_tan, fov_y_tan: fov_y_tan }
    }
}

impl Camera for PerspectiveCamera {
    fn cast(&self, film : &Film, fx : f32, fy : f32) -> Ray {
        let fw = film.width as f32;
        let fh = film.height as f32;
        let x = (fx / fw) * 2f32 - 1f32;
        let y = (fy / fh) * 2f32 - 1f32;
        let d = Vector::new(x * self.fov_x_tan, y * self.fov_y_tan, 1f32).normalize();
        let o = Point::origin();
        Ray::new(&o, &d).transform(&self.t.inverse())
    }
}

impl TransMut for PerspectiveCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
