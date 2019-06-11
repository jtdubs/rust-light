use crate::filters::filter::Filter;

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
            cache: [0f32; 256],
            x_scale: 15f32 / w,
            y_scale: (16f32 * 15f32) / h
        };
        for x in 0..16 {
            for y in 0..16 {
                let sx = (x as f32 / 15f32) * w;
                let sy = (y as f32 / 15f32) * h;
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
        let xa = x.abs();
        let ya = y.abs();
        if xa > self.w || ya > self.h {
            0f32
        } else {
            let sx = (xa * self.x_scale) as usize;
            let sy = (ya * self.y_scale) as usize;
            self.cache[sx + sy]
        }
    }
}
