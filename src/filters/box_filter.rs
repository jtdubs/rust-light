use crate::filters::filter::Filter;

pub struct BoxFilter {
    width : f32,
    height : f32,
}

impl BoxFilter {
    pub fn new(width : f32, height : f32) -> BoxFilter {
        BoxFilter { width: width, height: height }
    }
}

impl Filter for BoxFilter {
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        if x.abs() > self.width || y.abs() > self.height {
            0f32
        } else {
            1f32
        }
    }
}
