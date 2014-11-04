use shapes::shape::Shape;
use geometry::ray::Ray;
use aabb::AABB;

pub struct Primitive<'a> {
    pub shape : Box<Shape + 'a>,
}

impl<'a> Primitive<'a> {
    pub fn new(s : Box<Shape + 'a>) -> Primitive<'a> {
        Primitive { shape: s }
    }

    pub fn intersections(&self, r : &Ray) -> Vec<f32> {
        self.shape.intersections(r)
    }

    pub fn intersect(&self, r : &Ray) -> Option<f32> {
        self.shape.intersect(r)
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        self.shape.intersects(r)
    }

    pub fn bound(&self) -> AABB {
        self.shape.bound()
    }

    pub fn world_bound(&self) -> AABB {
        self.shape.world_bound()
    }
}

