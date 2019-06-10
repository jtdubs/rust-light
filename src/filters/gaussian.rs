use filters::filter::Filter;

pub struct GaussianFilter {
    width : f32,
    height : f32,
    alpha : f32,
    beta_x : f32,
    beta_y : f32,
}

impl GaussianFilter {
    pub fn new(width : f32, height : f32, alpha : f32) -> GaussianFilter {
        GaussianFilter { width: width, height: height, alpha: alpha, beta_x: (-alpha*width*width).exp(), beta_y: (-alpha*height*height).exp() }
    }
}

impl Filter for GaussianFilter {
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        if x.abs() > self.width || y.abs() > self.height {
            0f32
        } else {
            let gx = (-self.alpha * x * x).exp() - self.beta_x;
            let gy = (-self.alpha * y * y).exp() - self.beta_y;
            gx.max(0f32) * gy.max(0f32)
        }
    }
}
