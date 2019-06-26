use std::f32::consts::*;

use rand::prelude::*;
use rand::distributions::uniform::Uniform;

use crate::math::{radical_inverse, sobol, van_der_corput};

pub trait SamplerFactory1D {
    fn get_sampler(&self) -> Box<dyn Sampler1D>;
}

pub trait SamplerFactory2D {
    fn get_sampler(&self) -> Box<dyn Sampler2D>;
}


pub trait Sampler1D : Sync + Send  {
    fn get_samples(&mut self) -> Vec<f32>;
}

pub trait Sampler2D : Sync + Send {
    fn get_samples(&mut self) -> Vec<(f32, f32)>;
}


pub struct CentersSampler1D { }

impl CentersSampler1D {
    pub fn new() -> CentersSampler1D {
        CentersSampler1D { }
    }
}

impl Sampler1D for CentersSampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        vec![0.5f32]
    }
}


pub struct CentersSampler2D { }

impl CentersSampler2D {
    pub fn new() -> CentersSampler2D {
        CentersSampler2D { }
    }
}

impl Sampler2D for CentersSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        vec![(0.5f32, 0.5f32)]
    }
}


pub struct UniformSampler1D {
    range : Uniform<f32>,
    n : usize
}

impl UniformSampler1D {
    pub fn new(n : usize) -> UniformSampler1D {
        UniformSampler1D { range: Uniform::new_inclusive(0f32, 1f32), n: n }
    }
}

impl Sampler1D for UniformSampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        let mut rng = thread_rng();
        rng.sample_iter(&self.range).take(self.n).collect()
    }
}


pub struct UniformSampler2D {
    range : Uniform<f32>,
    n : usize
}

impl UniformSampler2D {
    pub fn new(n : usize) -> UniformSampler2D {
        UniformSampler2D { range: Uniform::new_inclusive(0f32, 1f32), n: n }
    }
}

impl Sampler2D for UniformSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();
        let l : Vec<f32> = rng.sample_iter(&self.range).take(self.n).collect();
        let r : Vec<f32> = rng.sample_iter(&self.range).take(self.n).collect();
        l.into_iter().zip(r.into_iter()).collect()
    }
}


pub struct StrataSampler1D {
    range : Uniform<f32>,
    n : usize

}

impl StrataSampler1D {
    pub fn new(n : usize) -> StrataSampler1D {
        StrataSampler1D { range: Uniform::new_inclusive(0f32, 1f32), n: n }
    }
}

impl Sampler1D for StrataSampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        let mut rng = thread_rng();
        let nf = self.n as f32;
        let ns = 1f32 / nf;

        let mut v = Vec::<f32>::with_capacity(self.n);
        for x in 0..self.n {
            let r = ns * (x as f32 + rng.sample(self.range));
            v.push(r);
        }
        v
    }
}


pub struct StrataSampler2D {
    range : Uniform<f32>,
    w : usize,
    h : usize
}

impl StrataSampler2D {
    pub fn new(w : usize, h : usize) -> StrataSampler2D {
        StrataSampler2D { range: Uniform::new_inclusive(0f32, 1f32), w: w, h: h }
    }
}

impl Sampler2D for StrataSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();
        let wf = self.w as f32;
        let hf = self.h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(self.w * self.h);
        for x in 0..self.w {
            for y in 0..self.h {
                let rx = (ws * (x as f32)) + (rng.sample(self.range) * ws);
                let ry = (hs * (y as f32)) + (rng.sample(self.range) * hs);
                v.push((rx, ry));
            }
        }
        v
    }
}


pub struct StrataCentersSampler1D {
    n : usize
}

impl StrataCentersSampler1D {
    pub fn new(n : usize) -> StrataCentersSampler1D {
        StrataCentersSampler1D { n: n }
    }
}

impl Sampler1D for StrataCentersSampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        let nf = self.n as f32;
        let ns = 1f32 / nf;
        let mut v = Vec::<f32>::with_capacity(self.n);
        for x in 0..self.n {
            let r = (ns * (x as f32)) + (ns / 2f32);
            v.push(r);
        }
        v
    }
}


pub struct StrataCentersSampler2D {
    w : usize,
    h : usize
}

impl StrataCentersSampler2D {
    pub fn new(w : usize, h : usize) -> StrataCentersSampler2D {
        StrataCentersSampler2D { w: w, h: h }
    }
}

impl Sampler2D for StrataCentersSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let wf = self.w as f32;
        let hf = self.h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(self.w * self.h);
        for x in 0..self.w {
            for y in 0..self.h {
                let rx = (ws * (x as f32)) + (ws / 2f32);
                let ry = (hs * (y as f32)) + (hs / 2f32);
                v.push((rx, ry));
            }
        }
        v
    }
}


pub struct HaltonSampler1D {
    n : usize
}

impl HaltonSampler1D {
    pub fn new(n : usize) -> HaltonSampler1D {
        HaltonSampler1D { n: n }
    }
}

impl Sampler1D for HaltonSampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(self.n);
        for x in 0..self.n {
            v.push(radical_inverse(x as u32, 2));
        }
        v
    }
}


pub struct HaltonSampler2D {
    n : usize
}

impl HaltonSampler2D {
    pub fn new(n : usize) -> HaltonSampler2D {
        HaltonSampler2D { n: n }
    }
}

impl Sampler2D for HaltonSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(self.n);
        for x in 0..self.n {
            v.push((radical_inverse(x as u32, 2), radical_inverse(x as u32, 3)));
        }
        v
    }
}


pub struct HammersleySampler1D {
    n : usize
}

impl HammersleySampler1D {
    pub fn new(n : usize) -> HammersleySampler1D {
        HammersleySampler1D { n: n }
    }
}

impl Sampler1D for HammersleySampler1D {
    fn get_samples(&mut self) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(self.n);
        for x in 0..self.n {
            v.push((x as f32) / (self.n as f32));
        }
        v
    }
}


pub struct HammersleySampler2D {
    n : usize
}

impl HammersleySampler2D {
    pub fn new(n : usize) -> HammersleySampler2D {
        HammersleySampler2D { n: n }
    }
}

impl Sampler2D for HammersleySampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(self.n);
        for x in 0..self.n {
            v.push((radical_inverse(x as u32, 2), (x as f32) / (self.n as f32)));
        }
        v
    }
}


pub struct LHCSampler2D {
    s : StrataSampler1D
}

impl LHCSampler2D {
    pub fn new(n : usize) -> LHCSampler2D {
        LHCSampler2D { s: StrataSampler1D::new(n) }
    }
}

impl Sampler2D for LHCSampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();

        let xs = self.s.get_samples();
        let mut ys = self.s.get_samples();
        ys.shuffle(&mut rng);

        xs.into_iter().zip(ys.into_iter()).collect()
    }
}


pub struct S02Sampler2D {
    s1 : u32,
    s2 : u32,
    n : usize,
}

impl S02Sampler2D {
    pub fn new(s1 : u32, s2 : u32, n : usize) -> S02Sampler2D {
        S02Sampler2D { s1: s1, s2: s2, n: n }
    }
}

impl Sampler2D for S02Sampler2D {
    fn get_samples(&mut self) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(self.n);
        for x in 0..self.n {
            v.push((van_der_corput(x as u32, self.s1), sobol(x as u32, self.s2)));
        }
        v
    }
}


pub fn to_disc_uniform((u1, u2) : (f32, f32)) -> (f32, f32) {
    let r = u1.sqrt();
    let theta = u2 * 2f32 * PI;
    (r * theta.cos(), r * theta.sin())
}

pub fn to_disc_concentric((u1, u2) : (f32, f32)) -> (f32, f32) {
    let a = (2f32 * u1) - 1f32;
    let b = (2f32 * u2) - 1f32;

    if a == 0f32 && b == 0f32 {
        (0f32, 0f32)
    } else {
        let (radius, theta) =
            if (a * a) > (b * b) {
                (a, FRAC_PI_4 * b / a)
            } else {
                (b, FRAC_PI_2 - FRAC_PI_4 * a / b)
            };

        (radius * theta.cos(), radius * theta.sin())
    }
}
