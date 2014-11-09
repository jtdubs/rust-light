// TODO: organize pixels in rectangular patches to improve cache coherency

extern crate lodepng;

use std::default::Default;

use filters::filter::Filter;

pub struct Pixel {
    sum : f32,
    weight_sum : f32
}

pub struct Film<'a> {
    pub width : uint,
    pub height : uint,
    filter : Box<Filter + 'a>,
    pixels : Vec<Pixel>,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel { sum: 0f32, weight_sum: 0f32 }
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new()
    }
}

impl<'a> Film<'a> {
    pub fn new(width : uint, height : uint, f : Box<Filter + 'a>) -> Film<'a> {
        Film {
            width: width, 
            height: height, 
            filter: f, 
            pixels: Vec::from_fn(width * height, |_| { Default::default() })
        }
    }

    pub fn new_1080(f : Box<Filter + 'a>) -> Film<'a> { Film::new(1920, 1080, f) }
    pub fn new_720(f : Box<Filter + 'a>) -> Film<'a> { Film::new(1280, 720, f) }
    pub fn new_480(f : Box<Filter + 'a>) -> Film<'a> { Film::new(720, 480, f) }
    pub fn new_2k(f : Box<Filter + 'a>) -> Film<'a> { Film::new(2048, 1080, f) }
    pub fn new_4k(f : Box<Filter + 'a>) -> Film<'a> { Film::new(4096, 2160, f) }
    pub fn new_8k(f : Box<Filter + 'a>) -> Film<'a> { Film::new(8192, 4608, f) }
    pub fn new_qvga(f : Box<Filter + 'a>) -> Film<'a> { Film::new(320, 240, f) }
    pub fn new_vga(f : Box<Filter + 'a>) -> Film<'a> { Film::new(640, 480, f) }

    pub fn sample_bounds(&self) -> ((int, int), (int, int)) {
        let (ex, ey) = self.filter.extent();
        (((-ex).floor() as int, (-ey).floor() as int), ((self.width as f32 + ex).ceil() as int, (self.height as f32 + ey).ceil() as int))
    }

    fn get_pixel(&self, x : uint, y : uint) -> &Pixel {
        &self.pixels[y * self.width + x]
    }

    fn get_pixel_mut(&mut self, x : uint, y : uint) -> &mut Pixel {
        &mut self.pixels[y * self.width + x]
    }

    // TODO: verify add_sample is walking the right range and picking the right weights
    pub fn add_sample(&mut self, x : f32, y : f32, v : u8) {
        let (ex, ey) = self.filter.extent();
        let min_x = (x + 0.5f32 - ex).ceil().max(0f32) as uint;
        let min_y = (y + 0.5f32 - ey).ceil().max(0f32) as uint;
        let max_x = (x - 0.5f32 + ex).floor().min(self.width as f32 - 1f32) as uint;
        let max_y = (y - 0.5f32 + ey).floor().min(self.height as f32 - 1f32) as uint;
        let ox = 0.5f32 - x;
        let oy = 0.5f32 - y;
        for ux in range(min_x, max_x+1) {
            for uy in range(min_y, max_y+1) {
                let w = self.filter.weight(ux as f32 + ox, uy as f32 + oy);
                let p = self.get_pixel_mut(ux, uy);
                p.sum = p.sum + (v as f32 * w);
                p.weight_sum = p.weight_sum + w;
            }
        }
    }

    pub fn save(&self, path : &Path) -> Result<(), &str> {
        let mut pixels = Vec::<u8>::new();
        for y in range(0, self.height) {
            for x in range(0, self.width) {
                let p = self.get_pixel(x, y);
                pixels.push((p.sum / p.weight_sum).round() as u8);
            }
        }

        match lodepng::encode_file(path, pixels.as_slice(), self.width as u32, self.height as u32, lodepng::LCT_GREY, 8) {
            Err(_) => Err("encoding failure"),
            Ok(_) => Ok(()),
        }
    }
}
