use geometry::transform::{Trans,TransMut};
use geometry::bounding_box::BoundingBox;
use geometry::ray::Ray;

pub trait Shape : Trans + TransMut {
    fn bound(&self) -> BoundingBox;
    fn world_bound(&self) -> BoundingBox;

    fn surface_area(&self) -> f32;

    fn intersections(&self, r : &Ray) -> Vec<f32>;
    fn intersect(&self, r : &Ray) -> Option<f32>;

    fn intersects(&self, r : &Ray) -> bool {
        match self.intersect(r) {
            None => false,
            Some(_) => true,
        }
    }
}

