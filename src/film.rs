use std::path::Path;
use std::default::Default;
use std::fs::File;

use image::png::PNGEncoder;
use image::ColorType;

pub struct Pixel {
    sum : f32,
    weight_sum : f32
}

pub struct Film {
    pub width : u32,
    pub height : u32,
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

impl Film {
    pub fn new(width : u32, height : u32) -> Film {
        let mut v : Vec<Pixel> = Vec::new();
        v.resize_with((width * height) as usize, || Default::default());
        Film {
            width: width, 
            height: height, 
            pixels: v
        }
    }

    pub fn new_1080() -> Film { Film::new(1920, 1080) }
    pub fn new_720() -> Film { Film::new(1280, 720) }
    pub fn new_480() -> Film { Film::new(720, 480) }
    pub fn new_2k() -> Film { Film::new(2048, 1080) }
    pub fn new_4k() -> Film { Film::new(4096, 2160) }
    pub fn new_8k() -> Film { Film::new(8192, 4608) }
    pub fn new_qvga() -> Film { Film::new(320, 240) }
    pub fn new_vga() -> Film { Film::new(640, 480) }

    #[inline]
    fn get_pixel_mut(&mut self, x : u32, y : u32) -> &mut Pixel {
        &mut self.pixels[((self.height - y - 1) * self.width + x) as usize]
    }

    #[inline]
    pub fn splat(&mut self, x : u32, y : u32, sum : f32, weight_sum : f32) {
        let p = self.get_pixel_mut(x, y);
        p.sum = p.sum + sum;
        p.weight_sum = p.weight_sum + weight_sum;
    }

    pub fn save(&self, path : &Path) -> Result<(), &str> {
        let pixels : Vec<u8> = self.pixels.iter().map(|p| (p.sum / p.weight_sum).round() as u8).collect();

        let file = File::create(path).unwrap();
        let encoder = PNGEncoder::new(file);

        match encoder.encode(pixels.as_slice(), self.width as u32, self.height as u32, ColorType::Gray(8)) {
            Ok(_)  => Ok(()),
            Err(_) => Err("save failed"),
        }
    }
}
