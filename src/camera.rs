use transform::{TransMut};
use ray::Ray;
use film::Film;

pub trait Camera : TransMut {
    fn cast(&self, film : &Film, fx : f32, fy : f32) -> Ray;
}
