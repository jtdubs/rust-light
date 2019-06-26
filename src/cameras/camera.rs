use crate::geometry::{Ray, HasTransform, TransMut};

pub trait Camera : HasTransform + TransMut + Send + Sync {
    fn cast(&self, x : f32, y : f32) -> Ray;
}
