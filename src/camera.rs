use transform::{TransMut};
use ray::Ray;

pub trait Camera : TransMut {
    fn cast(&self, x : f32, y : f32) -> Ray;
}
