use crate::filters::filter::Filter;

pub struct TriangleFilter {
    width  : f32,
    height : f32,
}
    
impl TriangleFilter {
    pub fn new(width : f32, height : f32) -> TriangleFilter {
        TriangleFilter { width: width, height: height }
    }
}

impl Filter for TriangleFilter {
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        let ax = x.abs();
        let ay = y.abs();

        if ax > self.width || ay > self.height {
            0f32
        } else {
            let tx = 1f32 - (ax / self.width);
            let ty = 1f32 - (ay / self.height);
            tx.max(0f32) * ty.max(0f32)
        }
    }
}
