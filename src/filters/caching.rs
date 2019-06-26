use crate::filters::Filter;

pub struct CachingFilter {
    width   : f32,
    height  : f32,
    cache   : [f32; 256],
    x_scale : f32,
    y_scale : f32,
}

impl CachingFilter {
    pub fn new(f : &dyn Filter) -> CachingFilter {
        let (width, height) = f.extent();
        let mut cf = CachingFilter {
            width: width,
            height: height, 
            cache: [0f32; 256],
            x_scale: 15f32 / width,
            y_scale: (16f32 * 15f32) / height
        };
        for x in 0..16 {
            for y in 0..16 {
                let sx = (x as f32 / 15f32) * width;
                let sy = (y as f32 / 15f32) * height;
                cf.cache[y * 16 + x] = f.weight(sx, sy);
            }
        };
        cf
    }
}

impl Filter for CachingFilter {
    #[inline]
    fn extent(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    #[inline]
    fn weight(&self, x : f32, y : f32) -> f32 {
        let xa = x.abs();
        let ya = y.abs();
        if xa > self.width || ya > self.height {
            0f32
        } else {
            let sx = (xa * self.x_scale) as usize;
            let sy = (ya * self.y_scale) as usize;
            self.cache[sx + sy]
        }
    }
}
