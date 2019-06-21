pub trait Filter : Sync + Send {
    fn extent(&self) -> (f32, f32);
    fn weight(&self, x : f32, y : f32) -> f32;
}
