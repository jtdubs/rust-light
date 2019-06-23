use crate::geometry::transform::{HasTransform,TransMut};
use crate::geometry::ray::Ray;

pub trait Camera : HasTransform + TransMut + Send + Sync {
    fn cast(&self, x : f32, y : f32) -> Ray;
}
