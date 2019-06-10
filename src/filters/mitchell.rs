use crate::filters::filter::Filter;

pub struct MitchellFilter {
    width : f32,
    height : f32,
    b : f32,
    c : f32,
}

impl MitchellFilter {
    pub fn new(width : f32, height : f32, b : f32, c : f32) -> MitchellFilter {
        MitchellFilter { width: width, height: height, b: b, c: c }
    }
}

impl Filter for MitchellFilter {
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        let b = self.b;
        let c = self.c;

        if x.abs() > self.width || y.abs() > self.height {
            0f32
        } else {
            let helper = |x : f32| -> f32 {
                let x2 = (2f32 * x).abs();
                if x2 > 1f32 {
                    ( ( -1f32 * b - 6f32 * c) * x2 * x2 * x2
                    + (  6f32 * b + 30f32 * c) * x2 * x2
                    + (-12f32 * b - 48f32 * c) * x2
                    + (  8f32 * b + 24f32 * c) ) / 6f32
                } else {
                    ( (  12f32 -  9f32 * b - 6f32 * c) * x2 * x2 * x2
                    + ( -18f32 + 12f32 * b + 6f32 * c) * x2 * x2
                    + (   6f32 -  2f32 * b) ) / 6f32
                }
            };

            helper(x / self.width) * helper(y / self.height)
        }
    }
}
