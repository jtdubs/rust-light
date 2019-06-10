use filters::filter::Filter;

pub struct CachingFilter {
    w : f32,
    h : f32,
    cache : [f32; 256],
    x_scale : f32,
    y_scale : f32,
}

impl CachingFilter {
    pub fn new(f : &Filter) -> CachingFilter {
        let (w, h) = f.extent();
        let mut cf = CachingFilter {
            w: w,
            h: h, 
            cache: [0f32, ..256] ,
            x_scale: 15f32 / w,
            y_scale: 15f32 * 16f32 / h,
        };
        for x in 0..16 {
            let fx = x as f32;
            for y in 0..16 {
                let fy = y as f32;
                let sx = (fx / 15f32) * w;
                let sy = (fy / 15f32) * h;
                cf.cache[y * 16 + x] = f.weight(sx, sy);
            }
        };
        cf
    }
}

impl Filter for CachingFilter {
    fn extent(&self) -> (f32, f32) {
        (self.w, self.h)
    }

    fn weight(&self, x : f32, y : f32) -> f32 {
        let sx = (x.abs() * self.x_scale) as u32;
        let sy = (y.abs() * self.y_scale) as u32;
        self.cache[sx + sy]
    }
}
