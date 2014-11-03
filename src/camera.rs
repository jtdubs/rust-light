use film::Film;
use transform::{Transform,Trans,TransMut};
use ray::Ray;
use vector::Vector;
use point::Point;

pub trait Camera : TransMut {
    fn get_film_size(&self) -> (u32, u32);

    fn cast(&self, fx : f32, fy : f32) -> Ray;
    fn receive(&mut self, fx : f32, fy : f32, p : u8);

    fn save(&self, p : &Path) -> Result<(), &str>;
}

pub struct OrthographicCamera {
    t : Transform,
    pub film : Box<Film>,
    pub scale : f32,
}

pub struct PerspectiveCamera {
    t : Transform,
    pub film : Box<Film>,
    pub fov_y : f32,
}

impl OrthographicCamera {
    pub fn new(film : Box<Film>, scale : f32) -> OrthographicCamera {
        OrthographicCamera { t: Transform::identity(), film: film, scale: scale }
    }
}

impl PerspectiveCamera {
    pub fn new(film : Box<Film>, fov_y : f32) -> PerspectiveCamera {
        PerspectiveCamera { t: Transform::identity(), film: film, fov_y: fov_y }
    }
}

impl Camera for OrthographicCamera {
    fn get_film_size(&self) -> (u32, u32) {
        (self.film.width, self.film.height)
    }

    fn cast(&self, fx : f32, fy : f32) -> Ray {
        let fw = self.film.width as f32;
        let fh = self.film.height as f32;
        let x = fx - (fw / 2f32);
        let y = fy - (fh / 2f32);
        let d = Vector::unit_z();
        let o = Point::new(x*self.scale, y*self.scale, 0f32);
        Ray::new(&o, &d).transform(&self.t.inverse())
    }

    fn receive(&mut self, fx : f32, fy : f32, p : u8) {
        self.film.add_sample(fx, fy, p)
    }

    fn save(&self, p : &Path) -> Result<(), &str> {
        self.film.save(p)
    }
}

impl Camera for PerspectiveCamera {
    fn get_film_size(&self) -> (u32, u32) {
        (self.film.width, self.film.height)
    }

    fn cast(&self, fx : f32, fy : f32) -> Ray {
        let fw = self.film.width as f32;
        let fh = self.film.height as f32;
        let x = (fx / fw) * 2f32 - 1f32;
        let y = (fy / fh) * 2f32 - 1f32;
        let sx = (self.fov_y / 2f32).tan() * (fw / fh);
        let sy = (self.fov_y / 2f32).tan();
        let d = Vector::new(x * sx, y * sy, 1f32).normalize();
        let o = Point::origin();
        Ray::new(&o, &d).transform(&self.t.inverse())
    }

    fn receive(&mut self, fx : f32, fy : f32, p : u8) {
        self.film.add_sample(fx, fy, p)
    }

    fn save(&self, p : &Path) -> Result<(), &str> {
        self.film.save(p)
    }
}

impl TransMut for OrthographicCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}

impl TransMut for PerspectiveCamera {
    fn transform_self(&mut self, t : &Transform) {
        self.t = t.compose(&self.t);
    }
}
