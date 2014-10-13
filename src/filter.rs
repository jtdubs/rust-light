use std::num::Float;

pub trait Filter {
    fn extent(&self) -> (f64, f64);
    fn weight(&self, x : f64, y : f64) -> f64;
}

pub struct BoxFilter {
  width  : f64,
  height : f64,
}

impl BoxFilter {
    pub fn new(width : f64, height : f64) -> BoxFilter {
        BoxFilter { width: width, height: height }
    }
}

impl Filter for BoxFilter {
    fn extent(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn weight(&self, x : f64, y : f64) -> f64 {
        if x.abs() > self.width || y.abs() > self.height {
            0f64
        } else {
            1f64
        }
    }
}

pub struct TriangleFilter {
    width  : f64,
    height : f64,
}

impl TriangleFilter {
    pub fn new(width : f64, height : f64) -> TriangleFilter {
        TriangleFilter { width: width, height: height }
    }
}

impl Filter for TriangleFilter {
    fn extent(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn weight(&self, x : f64, y : f64) -> f64 {
        if x.abs() > self.width || y.abs() > self.height {
            0f64
        } else {
            let tx = 1f64 - (x / self.width).abs();
            let ty = 1f64 - (y / self.height).abs();
            tx.max(0f64) * ty.max(0f64)
        }
    }
}

pub struct GaussianFilter {
    width  : f64,
    height : f64,
    alpha  : f64,
    base_x  : f64,
    base_y  : f64,
}

impl GaussianFilter {
    pub fn new(width : f64, height : f64, alpha : f64) -> GaussianFilter {
        GaussianFilter { width: width, height: height, alpha: alpha, base_x: (-alpha*width*width).exp(), base_y: (-alpha*height*height).exp() }
    }
}

impl Filter for GaussianFilter {
    fn extent(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn weight(&self, x : f64, y : f64) -> f64 {
        if x.abs() > self.width || y.abs() > self.height {
            0f64
        } else {
            let gx = (-self.alpha * x * x).exp() - self.base_x;
            let gy = (-self.alpha * y * y).exp() - self.base_y;
            gx.max(0f64) * gy.max(0f64)
        }
    }
}

pub struct MitchellFilter {
    width  : f64,
    height : f64,
    b      : f64,
    c      : f64,
}

impl MitchellFilter {
    pub fn new(width : f64, height : f64, b : f64, c : f64) -> MitchellFilter {
        MitchellFilter { width: width, height: height, b: b, c: c }
    }
    
    fn helper(&self, x : f64) -> f64 {
        let x2 = (2f64*x).abs();
        if x2 > 1f64 {
            ( ( -1f64 * self.b -  6f64 * self.c) * x2*x2*x2
            + (  6f64 * self.b + 30f64 * self.c) * x2*x2
            + (-12f64 * self.b - 48f64 * self.c) * x2
            + (  8f64 * self.b + 24f64 * self.c) ) / 6f64
        } else {
            ( (  12f64 -  9f64 * self.b - 6f64 * self.c) * x2*x2*x2
            + ( -18f64 + 12f64 * self.b + 6f64 * self.c) * x2*x2
            + (   6f64 -  2f64 * self.b) ) / 6f64
        }
    }
}

impl Filter for MitchellFilter {
    fn extent(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn weight(&self, x : f64, y : f64) -> f64 {
        if x.abs() > self.width || y.abs() > self.height {
            0f64
        } else {
            self.helper(x/self.width) * self.helper(y/self.height)
        }
    }
}

pub struct LanczosSincFilter {
    width  : f64,
    height : f64,
    tau    : f64,
}

impl LanczosSincFilter {
    pub fn new(width : f64, height : f64, tau : f64) -> LanczosSincFilter {
        LanczosSincFilter { width: width, height: height, tau: tau }
    }

    fn helper(&self, x : f64) -> f64 {
        if x < 0.00001f64 {
            1f64
        } else if x > 1f64 {
            0f64
        } else {
            let xp = x * Float::pi();
            let xpt = xp * self.tau;
            let sinc = xpt.sin() / xpt;
            let lanczos = xp.sin() / xp;
            sinc * lanczos
        }
    }
}

impl Filter for LanczosSincFilter {
    fn extent(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn weight(&self, x : f64, y : f64) -> f64 {
        if x.abs() > self.width || y.abs() > self.height {
            0f64
        } else {
            self.helper((x/self.width).abs()) * self.helper((y/self.height).abs())
        }
    }
}
