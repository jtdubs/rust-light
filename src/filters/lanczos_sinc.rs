use crate::filters::filter::Filter;

pub struct LanczosSincFilter {
    width : f32,
    height : f32,
    tau : f32,
}
    
impl LanczosSincFilter {
    pub fn new(width : f32, height : f32, tau : f32) -> LanczosSincFilter {
        LanczosSincFilter { width: width, height: height, tau: tau }
    }
}

impl Filter for LanczosSincFilter {
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        let ax = x.abs();
        let ay = y.abs();

        if ax > self.width || ay > self.height {
            0f32
        } else {
            let helper = |x : f32| -> f32 {
                if x < 0.00001f32 {
                    1f32
                } else if x > 1f32 {
                    0f32
                } else {
                    let xp = x * std::f32::consts::PI;
                    let xpt = xp * self.tau;
                    let sinc = xpt.sin() / xpt;
                    let lanczos = xp.sin() / xp;
                    sinc * lanczos
                }
            };

            helper(ax / self.width) * helper(ay / self.height)
        }
    }
}
