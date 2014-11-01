use shape::Shape;
use ray::Ray;
use aabb::AABB;

pub struct Primitive {
    pub shape : Shape,
}

impl Primitive {
    pub fn new(s : Shape) -> Primitive {
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

