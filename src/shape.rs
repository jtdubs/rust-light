use transform::{Trans,TransMut};
use aabb::AABB;
use ray::Ray;

pub trait Shape : Trans + TransMut {
    fn bound(&self) -> AABB;
    fn world_bound(&self) -> AABB;

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

