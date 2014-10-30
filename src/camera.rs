use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;

pub enum Camera {
    OrthoCamera(Transform, Box<Film>, f64),
    PerspectiveCamera(Transform, Box<Film>, f64),
}

impl Camera {
    pub fn new_ortho(f : Box<Film>, scale : f64) -> Camera {
        OrthoCamera(Transform::identity(), f, scale)
    }

    pub fn new_perspective(f : Box<Film>, fov_y : f64) -> Camera {
        PerspectiveCamera(Transform::identity(), f, fov_y)
    }

    pub fn get_film_size(&self) -> (u32, u32) {
        match self {
            &OrthoCamera(_, ref f, _) => (f.width, f.height),
            &PerspectiveCamera(_, ref f, _) => (f.width, f.height),
        }
    }

    pub fn cast(&self, fx : f64, fy : f64) -> Ray {
        match self {
            &OrthoCamera(ref t, ref f, s) => {
                let fw = f.width as f64;
                let fh = f.height as f64;
                let x = fx - (fw / 2f64);
                let y = fy - (fh / 2f64);
                let d = Vector::unit_z();
                let o = Point::new(x*s, y*s, 0f64);
                Ray::new(&o, &d).transform(&t.inverse())
            },
            &PerspectiveCamera(ref t, ref f, fov_y) => {
                let fw = f.width as f64;
                let fh = f.height as f64;
                let x = (fx / fw) * 2f64 - 1f64;
                let y = (fy / fh) * 2f64 - 1f64;
                let sx = (fov_y / 2f64).tan() * (fw / fh);
                let sy = (fov_y / 2f64).tan();
                let d = Vector::new(x * sx, y * sy, 1f64).normalize();
                let o = Point::origin();
                Ray::new(&o, &d).transform(&t.inverse())
            },
        }
    }

    pub fn receive(&mut self, fx : f64, fy : f64, p : u8) {
        match self {
            &OrthoCamera(_, ref mut f, _) => f.add_sample(fx, fy, p),
            &PerspectiveCamera(_, ref mut f, _) => f.add_sample(fx, fy, p),
        }
    }

    pub fn save(&self, path : &Path) -> Result<(), &str> {
        match self {
            &OrthoCamera(_, ref f, _) => f.save(path),
            &PerspectiveCamera(_, ref f, _) => f.save(path),
        }
    }            
}

impl TransMut for Camera {
    fn transform_self(&mut self, t : &Transform) {
        match self {
            &OrthoCamera(ref mut c, _, _) => { *c = t.compose(c); },
            &PerspectiveCamera(ref mut c, _, _) => { *c = t.compose(c); },
        }
    }
}
