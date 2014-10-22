use std::num::Float;

pub enum Filter {
    BoxFilter(f64, f64),
    TriangleFilter(f64, f64),
    GaussianFilter(f64, f64, f64, f64, f64),
    MitchellFilter(f64, f64, f64, f64),
    LanczosSincFilter(f64, f64, f64),
}
    
impl Filter {
    pub fn new_box(width : f64, height : f64) -> Filter {
        BoxFilter(width, height)
    }

    pub fn new_triangle(width : f64, height : f64) -> Filter {
        TriangleFilter(width, height)
    }

    pub fn new_gaussian(width : f64, height : f64, alpha : f64) -> Filter {
        GaussianFilter(width, height, alpha, (-alpha*width*width).exp(), (-alpha*height*height).exp())
    }

    pub fn new_mitchell(width : f64, height : f64, b : f64, c : f64) -> Filter {
        MitchellFilter(width, height, b, c)
    }

    pub fn new_lanczos_sinc(width : f64, height : f64, tau : f64) -> Filter {
        LanczosSincFilter(width, height, tau)
    }

    pub fn extent(&self) -> (f64, f64) {
        match *self {
            BoxFilter(w, h) => (w, h),
            TriangleFilter(w, h) => (w, h),
            GaussianFilter(w, h, _, _, _) => (w, h),
            MitchellFilter(w, h, _, _) => (w, h),
            LanczosSincFilter(w, h, _) => (w, h),
        }
    }

    pub fn weight(&self, x : f64, y : f64) -> f64 {
        let (w, h) = self.extent();
        if x.abs() > w || y.abs() > h {
            0f64
        } else {
            match *self {
                BoxFilter(_, _) => 1f64,
                TriangleFilter(w, h) => {
                    let tx = 1f64 - (x / w).abs();
                    let ty = 1f64 - (y / h).abs();
                    tx.max(0f64) * ty.max(0f64)
                },
                GaussianFilter(_, _, a, bx, by) => {
                    let gx = (-a * x * x).exp() - bx;
                    let gy = (-a * y * y).exp() - by;
                    gx.max(0f64) * gy.max(0f64)
                },
                MitchellFilter(w, h, b, c) => {
                    let helper = |x : f64| -> f64 {
                        let x2 = (2f64*x).abs();
                        if x2 > 1f64 {
                            ( ( -1f64 * b -  6f64 * c) * x2 * x2 * x2
                            + (  6f64 * b + 30f64 * c) * x2 * x2
                            + (-12f64 * b - 48f64 * c) * x2
                            + (  8f64 * b + 24f64 * c) ) / 6f64
                        } else {
                            ( (  12f64 -  9f64 * b - 6f64 * c) * x2 * x2 * x2
                            + ( -18f64 + 12f64 * b + 6f64 * c) * x2 * x2
                            + (   6f64 -  2f64 * b) ) / 6f64
                        }
                    };

                    helper(x/w) * helper(y/h)
                },
                LanczosSincFilter(w, h, t) => {
                    let helper = |x : f64| -> f64 {
                        if x < 0.00001f64 {
                            1f64
                        } else if x > 1f64 {
                            0f64
                        } else {
                            let xp = x * Float::pi();
                            let xpt = xp * t;
                            let sinc = xpt.sin() / xpt;
                            let lanczos = xp.sin() / xp;
                            sinc * lanczos
                        }
                    };

                    helper((x/w).abs()) * helper((y/h).abs())
                },
            }
        }
    }
}
