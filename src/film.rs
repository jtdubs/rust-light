extern crate lodepng;

use filter::Filter;

pub struct Pixel {
    sum : f32,
    weight_sum : f32
}

pub struct Film {
    pub width : u32,
    pub height : u32,
    filter : Filter,
    pixels : Vec<Pixel>,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { sum: 0f32, weight_sum: 0f32 }
    }
}

impl Film {
    pub fn new(width : u32, height : u32, f : Filter) -> Film {
        Film { width: width, height: height, filter: f, pixels: Vec::<Pixel>::from_fn((width * height) as uint, |_| Pixel::new()) }
    }

    pub fn new_1080(f : Filter) -> Film { Film::new(1920, 1080, f) }
    pub fn new_720(f : Filter) -> Film { Film::new(1280, 720, f) }
    pub fn new_480(f : Filter) -> Film { Film::new(720, 480, f) }
    pub fn new_2k(f : Filter) -> Film { Film::new(2048, 1080, f) }
    pub fn new_4k(f : Filter) -> Film { Film::new(4096, 2160, f) }
    pub fn new_8k(f : Filter) -> Film { Film::new(8192, 4608, f) }
    pub fn new_qvga(f : Filter) -> Film { Film::new(320, 240, f) }
    pub fn new_vga(f : Filter) -> Film { Film::new(640, 480, f) }

    pub fn sample_bounds(&self) -> ((int, int), (int, int)) {
        let (ex, ey) = self.filter.extent();
        (((-ex).floor() as int, (-ey).floor() as int), ((self.width as f32 + ex).ceil() as int, (self.height as f32 + ey).ceil() as int))
    }

    fn get_pixel(&mut self, x : u32, y : u32) -> &mut Pixel {
        self.pixels.get_mut((y * self.width + x) as uint)
    }

    pub fn add_sample(&mut self, x : f32, y : f32, v : u8) {
        let (ex, ey) = self.filter.extent();
        let min_x = (x - 0.5f32 - ex).ceil().max(0f32) as u32;
        let min_y = (y - 0.5f32 - ey).ceil().max(0f32) as u32;
        let max_x = (x - 0.5f32 + ex).floor().min(self.width as f32 - 1f32) as u32;
        let max_y = (y - 0.5f32 + ey).floor().min(self.height as f32 - 1f32) as u32;
        for ux in range(min_x, max_x+1) {
            for uy in range(min_y, max_y+1) {
                let w = self.filter.weight(ux as f32 - x - 0.5, uy as f32 - y - 0.5);
                let p = self.get_pixel(ux, uy);
                p.sum = p.sum + (v as f32 * w);
                p.weight_sum = p.weight_sum + w;
            }
        }
    }

    pub fn save(&self, path : &Path) -> Result<(), &str> {
        match lodepng::encode_file(path, self.pixels.iter().map(|p| { (p.sum / p.weight_sum).round() as u8 }).collect::<Vec<u8>>().as_slice(), self.width, self.height, lodepng::LCT_GREY, 8) {
            Err(_) => Err("encoding failure"),
            Ok(_) => Ok(()),
        }
    }
}
