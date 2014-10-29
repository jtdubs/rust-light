use shape::Shape;
use ray::Ray;

pub struct Primitive {
    pub shape : Shape,
}

impl Primitive {
    pub fn new(s : Shape) -> Primitive {
        Primitive { shape: s }
    }

    pub fn intersections(&self, r : &Ray) -> Vec<f64> {
        self.shape.intersections(r)
    }

    pub fn intersect(&self, r : &Ray) -> Option<f64> {
        self.shape.intersect(r)
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        self.shape.intersects(r)
    }
}

