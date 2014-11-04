use shapes::shape::{Shape,Intersection};
use geometry::ray::Ray;
use geometry::bounding_box::BoundingBox;
use std::rc::Rc;

pub struct Primitive<'a> {
    pub shape : Rc<Box<Shape + 'a>>,
}

impl<'a> Primitive<'a> {
    pub fn new<'a>(s : Box<Shape + 'a>) -> Primitive<'a> {
        Primitive { shape: Rc::new(s) }
    }

    pub fn intersections(&self, r : &Ray) -> Vec<Intersection> {
        self.shape.intersections(r)
    }

    pub fn intersect(&self, r : &Ray) -> Option<Intersection> {
        self.shape.intersect(r)
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        self.shape.intersects(r)
    }

    pub fn bound(&self) -> BoundingBox {
        self.shape.bound()
    }

    pub fn world_bound(&self) -> BoundingBox {
        self.shape.world_bound()
    }
}

