use filters::filter::Filter;

pub struct CachingFilter {
    w : f32,
    h : f32,
    cache : [f32, ..256]
}

impl CachingFilter {
    pub fn new(f : &Filter) -> CachingFilter {
        let (w, h) = f.extent();
        let mut cf = CachingFilter { w: w, h: h, cache: [0f32, ..256] };
        for x in range(0u, 16u) {
            let fx = x as f32;
            for y in range(0u, 16u) {
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
        let sx = (x.abs() * 15f32 / self.w) as uint;
        let sy = (y.abs() * 15f32 / self.h) as uint;
        if sx > 15 || sy > 15 { 0f32 } else { self.cache[sy * 16 + sx] }
    }
}
