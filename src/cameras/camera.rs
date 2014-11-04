use geometry::transform::{TransMut};
use geometry::ray::Ray;

pub trait Camera : TransMut {
    fn cast(&self, x : f32, y : f32) -> Ray;
}
