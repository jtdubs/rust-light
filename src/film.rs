// TODO: organize pixels in rectangular patches to improve cache coherency

extern crate png;

use std::default::Default;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

use crate::filters::filter::Filter;

pub struct Pixel {
    sum : f32,
    weight_sum : f32
}

pub struct Film<F> where F : Filter {
    pub width : u32,
    pub height : u32,
    filter : F,
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

impl<F : Filter> Film<F> {
    pub fn new(width : u32, height : u32, f : F) -> Film<F> {
        let mut v : Vec<Pixel> = Vec::new();
        v.resize_with((width * height) as usize, || Default::default());
        Film {
            width: width, 
            height: height, 
            filter: f, 
            pixels: v
        }
    }

    pub fn new_1080(f : F) -> Film<F> { Film::new(1920, 1080, f) }
    pub fn new_720(f : F) -> Film<F> { Film::new(1280, 720, f) }
    pub fn new_480(f : F) -> Film<F> { Film::new(720, 480, f) }
    pub fn new_2k(f : F) -> Film<F> { Film::new(2048, 1080, f) }
    pub fn new_4k(f : F) -> Film<F> { Film::new(4096, 2160, f) }
    pub fn new_8k(f : F) -> Film<F> { Film::new(8192, 4608, f) }
    pub fn new_qvga(f : F) -> Film<F> { Film::new(320, 240, f) }
    pub fn new_vga(f : F) -> Film<F> { Film::new(640, 480, f) }

    pub fn sample_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let (ex, ey) = self.filter.extent();
        (((-ex).floor() as i32, (-ey).floor() as i32), ((self.width as f32 + ex).ceil() as i32, (self.height as f32 + ey).ceil() as i32))
    }

    // fn get_pixel(&self, x : u32, y : u32) -> &Pixel {
    //     &self.pixels[(y * self.width + x) as usize]
    // }

    #[inline]
    fn get_pixel_mut(&mut self, x : u32, y : u32) -> &mut Pixel {
        &mut self.pixels[(y * self.width + x) as usize]
    }

    // TODO: verify add_sample is walking the right range and picking the right weights
    #[inline]
    pub fn add_sample(&mut self, x : f32, y : f32, v : u8) {
        let (ex, ey) = self.filter.extent();
        let min_x = (x - 0.5f32 - ex).ceil().max(0f32) as u32;
        let min_y = (y - 0.5f32 - ey).ceil().max(0f32) as u32;
        let max_x = (x + 0.5f32 + ex).min(self.width as f32 - 1f32) as u32;
        let max_y = (y + 0.5f32 + ey).min(self.height as f32 - 1f32) as u32;
        let ox = 0.5f32 - x;
        let oy = 0.5f32 - y;
        let vf = v as f32;
        for ux in min_x..=max_x {
            for uy in min_y..=max_y {
                let w = self.filter.weight(ux as f32 + ox, uy as f32 + oy);
                let p = self.get_pixel_mut(ux, uy);
                p.sum = p.sum + (vf * w);
                p.weight_sum = p.weight_sum + w;
            }
        }
    }

    pub fn save(&self, path : &Path) -> Result<(), &str> {
        let pixels : Vec<u8> = self.pixels.iter().map(|p| (p.sum / p.weight_sum).round() as u8).collect();

        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        match writer.write_image_data(&pixels) {
            Ok(_)  => Ok(()),
            Err(_) => Err("save failed"),
        }
    }
}
