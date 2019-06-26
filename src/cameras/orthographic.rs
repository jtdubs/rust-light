use crate::geometry::{Ray, Vector, Point, Transform, HasTransform, TransMut};
use crate::cameras::Camera;

pub struct OrthographicCamera {
    transform : Transform,
    pub scale : f32,
    pub aspect_ratio : f32
}

impl OrthographicCamera {
    pub fn new(scale : f32, aspect_ratio : f32) -> OrthographicCamera {
        OrthographicCamera { transform: Transform::identity(), scale: scale, aspect_ratio: aspect_ratio }
    }
}

impl Camera for OrthographicCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let o = Point::new(x * self.scale * self.aspect_ratio, y * self.scale, 0f32);
        Ray::new(&o, &Vector::unit_z()).from(self)
    }
}

impl HasTransform for OrthographicCamera {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl TransMut for OrthographicCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
