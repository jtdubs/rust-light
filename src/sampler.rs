extern crate rand;

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::distributions::uniform::{Uniform};

use crate::math::{radical_inverse,sobol,van_der_corput};

pub struct Sampler {
    range : Uniform<f32>
}

impl Sampler {
    pub fn new() -> Sampler {
        Sampler { range: Uniform::new_inclusive(0f32, 1f32) }
    }

    pub fn uniform_1d(&mut self, n : u32) -> Vec<f32> {
        let mut rng = thread_rng();
        rng.sample_iter(&self.range).take(n as usize).collect()
    }

    pub fn uniform_2d(&mut self, n : u32) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();
        let l : Vec<f32> = rng.sample_iter(&self.range).take(n as usize).collect();
        let r : Vec<f32> = rng.sample_iter(&self.range).take(n as usize).collect();
        l.into_iter().zip(r.into_iter()).collect()
    }

    pub fn strata_1d(&mut self, n : u32) -> Vec<f32> {
        let mut rng = thread_rng();
        let nf = n as f32;
        let ns = 1f32 / nf;

        let mut v = Vec::<f32>::with_capacity(n as usize);
        for x in 0..n {
            let r = ns * (x as f32 + rng.sample(self.range));
            v.push(r);
        }
        v
    }

    pub fn strata_2d(&mut self, w : u32, h : u32) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();
        let wf = w as f32;
        let hf = h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(w as usize * h as usize);
        for x in 0..w {
            for y in 0..h {
                let rx = (ws * (x as f32)) + (rng.sample(self.range) * ws);
                let ry = (hs * (y as f32)) + (rng.sample(self.range) * hs);
                v.push((rx, ry));
            }
        }
        v
    }

    pub fn strata_centers_1d(&mut self, n : u32) -> Vec<f32> {
        let nf = n as f32;
        let ns = 1f32 / nf;
        let mut v = Vec::<f32>::with_capacity(n as usize);
        for x in 0..n {
            let r = (ns * (x as f32)) + (ns / 2f32);
            v.push(r);
        }
        v
    }

    pub fn strata_centers_2d(&mut self, w : u32, h : u32) -> Vec<(f32, f32)> {
        let wf = w as f32;
        let hf = h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(w as usize * h as usize);
        for x in 0..w {
            for y in 0..h {
                let rx = (ws * (x as f32)) + (ws / 2f32);
                let ry = (hs * (y as f32)) + (hs / 2f32);
                v.push((rx, ry));
            }
        }
        v
    }

    pub fn halton_1d(&mut self, n : u32) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(n as usize);
        for x in 0..n {
            v.push(radical_inverse(x, 2));
        }
        v
    }

    pub fn halton_2d(&mut self, n : u32) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n as usize);
        for x in 0..n {
            v.push((radical_inverse(x, 2), radical_inverse(x, 3)));
        }
        v
    }

    pub fn hammersley_1d(&mut self, n : u32) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(n as usize);
        for x in 0..n {
            v.push((x as f32) / (n as f32));
        }
        v
    }

    pub fn hammersley_2d(&mut self, n : u32) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n as usize);
        for x in 0..n {
            v.push((radical_inverse(x, 2), (x as f32) / (n as f32)));
        }
        v
    }

    pub fn lhc_2d(&mut self, n : u32) -> Vec<(f32, f32)> {
        let mut rng = thread_rng();

        let xs = self.strata_1d(n);
        let mut ys = self.strata_1d(n);
        ys.shuffle(&mut rng);

        xs.into_iter().zip(ys.into_iter()).collect()
    }

    pub fn s02_2d(&mut self, s1 : u32, s2 : u32, n : u32) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n as usize);
        for x in 0..n {
            v.push((van_der_corput(x as u32, s1), sobol(x as u32, s2)));
        }
        v
    }
}
