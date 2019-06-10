use crate::geometry::transform::{TransMut};
use crate::geometry::ray::Ray;

pub trait Camera : TransMut {
    fn cast(&self, x : f32, y : f32) -> Ray;
}
