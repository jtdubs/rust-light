use crate::geometry::transform::{Transform,HasTransform,TransMut};
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::point::Point;
use crate::cameras::camera::Camera;

pub struct SphereCamera {
    transform : Transform,
}

impl SphereCamera {
    pub fn new() -> SphereCamera {
        SphereCamera { transform: Transform::identity() }
    }
}

impl Camera for SphereCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let h = x * core::f32::consts::PI;
        let v = y * core::f32::consts::FRAC_PI_2;

        let d = Vector::new(h.sin() * v.cos(), v.sin(), h.cos() * v.cos());
        Ray::new(&Point::origin(), &d).from(self)


/*                                       h  h.sin()  h.cos()      v  v.sin()  v.cos()
            [-1, -1] -> ( 0, -1, 0)  -pi/2       -1        0  -pi/2       -1        0
            [-1,  0] -> (-1,  0, 0)  -pi/2       -1        0      0        0        1
            [-1,  1] -> ( 0,  1, 0)  -pi/2       -1        0   pi/2        1        0
            [ 0, -1] -> ( 0, -1, 0)      0        0        1  -pi/2       -1        0
            [ 0,  0] -> ( 0,  0, 1)      0        0        1      0        0        1
            [ 0,  1] -> ( 0,  1, 0)      0        0        1   pi/2        1        0
            [ 1, -1] -> ( 0, -1  0)   pi/2        1        0  -pi/2       -1        0
            [ 1,  0] -> ( 1,  0, 0)   pi/2        1        0      0        0        1
            [ 1,  1] -> ( 0,  1, 0)   pi/2        1        0   pi/2        1        0
*/

    }
}

impl HasTransform for SphereCamera {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl TransMut for SphereCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
