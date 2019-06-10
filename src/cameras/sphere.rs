use crate::geometry::transform::{Transform,Trans,TransMut};
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::point::Point;
use crate::cameras::camera::Camera;

pub struct SphereCamera {
    t : Transform,
}

impl SphereCamera {
    pub fn new() -> SphereCamera {
        SphereCamera { t: Transform::identity() }
    }
}

impl Camera for SphereCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let h = x * core::f32::consts::PI;
        let v = y * core::f32::consts::FRAC_PI_2;

        let d = Vector::new(h.sin() * v.cos(), v.sin(), h.cos() * v.cos());
        Ray::new(&Point::origin(), &d).transform(&-self.t)


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

impl TransMut for SphereCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
