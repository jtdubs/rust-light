use crate::geometry::{Ray, Vector, Point, Transform, HasTransform, TransMut};
use crate::cameras::Camera;

pub struct PerspectiveCamera {
    transform : Transform,
    pub fov_y : f32,
    fov_x_tan : f32,
    fov_y_tan : f32,
}

impl PerspectiveCamera {
    pub fn new(fov_y : f32, aspect_ratio : f32) -> PerspectiveCamera {
        let fov_y_tan = (fov_y / 2f32).tan();
        let fov_x_tan = fov_y_tan * aspect_ratio;
        PerspectiveCamera { transform: Transform::identity(), fov_y: fov_y, fov_x_tan: fov_x_tan, fov_y_tan: fov_y_tan }
    }
}

impl Camera for PerspectiveCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let d = Vector::new(x * self.fov_x_tan, y * self.fov_y_tan, 1f32).normalize();
        Ray::new(&Point::origin(), &d).from(self)
    }
}

impl HasTransform for PerspectiveCamera {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl TransMut for PerspectiveCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
