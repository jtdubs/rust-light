use crate::geometry::{Ray, Vector, Point, Transform, HasTransform, TransMut};
use crate::cameras::Camera;
use crate::sampler::{UniformSampler2D, Sampler2D, to_disc_concentric};

#[derive(Debug)]
pub struct PerspectiveLensCamera {
    transform      : Transform,
    pub fov_y      : f32,
    fov_x_tan      : f32,
    fov_y_tan      : f32,
    lens_radius    : f32,
    focal_distance : f32,
}

impl PerspectiveLensCamera {
    pub fn new(fov_y : f32, aspect_ratio : f32, lens_radius : f32, focal_distance : f32) -> PerspectiveLensCamera {
        let fov_y_tan = (fov_y / 2f32).tan();
        let fov_x_tan = fov_y_tan * aspect_ratio;

        PerspectiveLensCamera {
            transform:      Transform::identity(),
            fov_y:          fov_y,
            fov_x_tan:      fov_x_tan,
            fov_y_tan:      fov_y_tan,
            lens_radius:    lens_radius,
            focal_distance: focal_distance,
        }
    }
}

impl Camera for PerspectiveLensCamera {
    fn cast(&self, x : f32, y : f32) -> Ray {
        let mut sampler = UniformSampler2D::new(1);

        let d = Vector::new(x * self.fov_x_tan, y * self.fov_y_tan, 1f32).normalize();
        let r = Ray::new(&Point::origin(), &d);
        
        if self.lens_radius <= 0f32 {
            return r.from(self);
        }

        let s = sampler.get_samples()[0];
        let (mut u, mut v) = to_disc_concentric(s);
        u *= self.lens_radius;
        v *= self.lens_radius;

        let focal_point = r.at_time(self.focal_distance / r.direction.z);

        let lens_origin = Point::new(u, v, 0f32);
        let lens_dir    = focal_point - lens_origin;
        Ray::new(&lens_origin, &lens_dir.normalize()).from(self)
    }
}

impl HasTransform for PerspectiveLensCamera {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl TransMut for PerspectiveLensCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.transform = *t + self.transform;
    }
}
