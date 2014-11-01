use std::num::Float;

pub enum Filter {
    BoxFilter(f32, f32),
    TriangleFilter(f32, f32),
    GaussianFilter(f32, f32, f32, f32, f32),
    MitchellFilter(f32, f32, f32, f32),
    LanczosSincFilter(f32, f32, f32),
}
    
impl Filter {
    pub fn new_box(width : f32, height : f32) -> Filter {
        BoxFilter(width, height)
    }

    pub fn new_triangle(width : f32, height : f32) -> Filter {
        TriangleFilter(width, height)
    }

    pub fn new_gaussian(width : f32, height : f32, alpha : f32) -> Filter {
        GaussianFilter(width, height, alpha, (-alpha*width*width).exp(), (-alpha*height*height).exp())
    }

    pub fn new_mitchell(width : f32, height : f32, b : f32, c : f32) -> Filter {
        MitchellFilter(width, height, b, c)
    }

    pub fn new_lanczos_sinc(width : f32, height : f32, tau : f32) -> Filter {
        LanczosSincFilter(width, height, tau)
    }

    pub fn extent(&self) -> (f32, f32) {
        match *self {
            BoxFilter(w, h) => (w, h),
            TriangleFilter(w, h) => (w, h),
            GaussianFilter(w, h, _, _, _) => (w, h),
            MitchellFilter(w, h, _, _) => (w, h),
            LanczosSincFilter(w, h, _) => (w, h),
        }
    }

    pub fn weight(&self, x : f32, y : f32) -> f32 {
        let (w, h) = self.extent();
        if x.abs() > w || y.abs() > h {
            0f32
        } else {
            match *self {
                BoxFilter(_, _) => 1f32,
                TriangleFilter(w, h) => {
                    let tx = 1f32 - (x / w).abs();
                    let ty = 1f32 - (y / h).abs();
                    tx.max(0f32) * ty.max(0f32)
                },
                GaussianFilter(_, _, a, bx, by) => {
                    let gx = (-a * x * x).exp() - bx;
                    let gy = (-a * y * y).exp() - by;
                    gx.max(0f32) * gy.max(0f32)
                },
                MitchellFilter(w, h, b, c) => {
                    let helper = |x : f32| -> f32 {
                        let x2 = (2f32*x).abs();
                        if x2 > 1f32 {
                            ( ( -1f32 * b -  6f32 * c) * x2 * x2 * x2
                            + (  6f32 * b + 30f32 * c) * x2 * x2
                            + (-12f32 * b - 48f32 * c) * x2
                            + (  8f32 * b + 24f32 * c) ) / 6f32
                        } else {
                            ( (  12f32 -  9f32 * b - 6f32 * c) * x2 * x2 * x2
                            + ( -18f32 + 12f32 * b + 6f32 * c) * x2 * x2
                            + (   6f32 -  2f32 * b) ) / 6f32
                        }
                    };

                    helper(x/w) * helper(y/h)
                },
                LanczosSincFilter(w, h, t) => {
                    let helper = |x : f32| -> f32 {
                        if x < 0.00001f32 {
                            1f32
                        } else if x > 1f32 {
                            0f32
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
