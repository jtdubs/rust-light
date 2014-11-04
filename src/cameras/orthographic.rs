use geometry::transform::{Transform,Trans,TransMut};
use geometry::ray::Ray;
use geometry::vector::Vector;
use geometry::point::Point;
use cameras::camera::Camera;

pub struct OrthographicCamera {
    t : Transform,
    pub scale : f32,
    pub aspect_ratio : f32
}

impl OrthographicCamera {
    pub fn new(scale : f32, aspect_ratio : f32) -> OrthographicCamera {
        OrthographicCamera { t: Transform::identity(), scale: scale, aspect_ratio: aspect_ratio }
    }
}

impl Camera for OrthographicCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let o = Point::new(x * self.scale * self.aspect_ratio, y * self.scale, 0f32);
        Ray::new(&o, &Vector::unit_z()).transform(&self.t.inverse())
    }
}

impl TransMut for OrthographicCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
