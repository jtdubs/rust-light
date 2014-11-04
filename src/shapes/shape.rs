use geometry::transform::{Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;
use geometry::point::Point;

pub struct Intersection {
    pub ray : Ray,
    pub time : f32,
    pub point : Point,
}

impl Intersection {
    pub fn new(ray : &Ray, time : f32, point : &Point) -> Intersection {
        Intersection { ray: *ray, time: time, point: *point }
    }
}

pub trait Shape : Trans + TransMut {
    fn bound(&self) -> BoundingBox;
    fn world_bound(&self) -> BoundingBox;

    fn surface_area(&self) -> f32;

    fn intersections(&self, r : &Ray) -> Vec<Intersection>;
    fn intersect(&self, r : &Ray) -> Option<Intersection>;

    fn intersects(&self, r : &Ray) -> bool {
        match self.intersect(r) {
            None => false,
            Some(_) => true,
        }
    }
}

