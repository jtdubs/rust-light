use geometry::transform::{Transform,TransMut};
use geometry::ray::Ray;
use geometry::vector::Vector;
use geometry::point::Point;
use cameras::camera::Camera;

pub struct HemisphereCamera {
    t : Transform,
}

impl HemisphereCamera {
    pub fn new() -> HemisphereCamera {
        HemisphereCamera { t: Transform::identity() }
    }
}

impl Camera for HemisphereCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let h = x * Float::frac_pi_2();
        let v = y * Float::frac_pi_2();

        let d = Vector::new(h.sin() * v.cos(), v.sin(), h.cos() * v.cos());
        Ray::new(&Point::origin(), &d) * -self.t


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

impl TransMut for HemisphereCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = *t + self.t;
    }
}
