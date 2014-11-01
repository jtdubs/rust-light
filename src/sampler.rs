extern crate std;

use std::rand::distributions::{Range, Sample};
use std::rand::{XorShiftRng, Rng};

use math::{radical_inverse,sobol,van_der_corput};

pub struct Sampler {
    rng : XorShiftRng,
    range : Range<f32>,
}

impl Sampler {
    pub fn new() -> Sampler {
        Sampler { rng: XorShiftRng::new_unseeded(), range: Range::new(0f32, 1f32) }
    }

    pub fn uniform_1d(&mut self, n : uint) -> Vec<f32> {
        let rng = &mut self.rng;
        let mut r = Vec::<f32>::with_capacity(n);
        for _ in std::iter::range(0u, n) {
            r.push(self.range.sample::<XorShiftRng>(rng));
        }
        r
    }

    pub fn uniform_2d(&mut self, n : uint) -> Vec<(f32, f32)> {
        let rng = &mut self.rng;
        let mut r = Vec::<(f32, f32)>::with_capacity(n);
        for _ in std::iter::range(0u, n) {
            r.push((self.range.sample::<XorShiftRng>(rng), self.range.sample::<XorShiftRng>(rng)));
        }
        r
    }

    pub fn strata_1d(&mut self, n : uint) -> Vec<f32> {
        let rng = &mut self.rng;
        let nf = n as f32;
        let ns = 1f32 / nf;
        let mut v = Vec::<f32>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            let r = (ns * (x as f32)) + (self.range.sample::<XorShiftRng>(rng) * ns);
            v.push(r);
        }
        v
    }

    pub fn strata_2d(&mut self, w : uint, h : uint) -> Vec<(f32, f32)> {
        let rng = &mut self.rng;
        let wf = w as f32;
        let hf = h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(w * h);
        for x in std::iter::range(0u, w) {
            for y in std::iter::range(0u, h) {
                let rx = (ws * (x as f32)) + (self.range.sample::<XorShiftRng>(rng) * ws);
                let ry = (hs * (y as f32)) + (self.range.sample::<XorShiftRng>(rng) * hs);
                v.push((rx, ry));
            }
        }
        v
    }

    pub fn strata_centers_1d(&mut self, n : uint) -> Vec<f32> {
        let nf = n as f32;
        let ns = 1f32 / nf;
        let mut v = Vec::<f32>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            let r = (ns * (x as f32)) + (ns / 2f32);
            v.push(r);
        }
        v
    }

    pub fn strata_centers_2d(&mut self, w : uint, h : uint) -> Vec<(f32, f32)> {
        let wf = w as f32;
        let hf = h as f32;
        let ws = 1f32 / wf;
        let hs = 1f32 / hf;
        let mut v = Vec::<(f32, f32)>::with_capacity(w * h);
        for x in std::iter::range(0u, w) {
            for y in std::iter::range(0u, h) {
                let rx = (ws * (x as f32)) + (ws / 2f32);
                let ry = (hs * (y as f32)) + (hs / 2f32);
                v.push((rx, ry));
            }
        }
        v
    }

    pub fn halton_1d(&mut self, n : uint) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push(radical_inverse(x, 2));
        }
        v
    }

    pub fn halton_2d(&mut self, n : uint) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push((radical_inverse(x, 2), radical_inverse(x, 3)));
        }
        v
    }

    pub fn hammersley_1d(&mut self, n : uint) -> Vec<f32> {
        let mut v = Vec::<f32>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push((x as f32) / (n as f32));
        }
        v
    }

    pub fn hammersley_2d(&mut self, n : uint) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push((radical_inverse(x, 2), (x as f32) / (n as f32)));
        }
        v
    }

    pub fn lhc_2d(&mut self, n : uint) -> Vec<(f32, f32)> {
        let xs = self.strata_1d(n);
        let mut ys = self.strata_1d(n);
        let ys2 = ys.as_mut_slice();
        self.rng.shuffle(ys2);
        
        let mut v = Vec::<(f32, f32)>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push((xs[x], ys2[x]));
        }
        v
    }

    pub fn s02_2d(&mut self, s1 : u32, s2 : u32, n : uint) -> Vec<(f32, f32)> {
        let mut v = Vec::<(f32, f32)>::with_capacity(n);
        for x in std::iter::range(0u, n) {
            v.push((van_der_corput(x as u32, s1), sobol(x as u32, s2)));
        }
        v
    }
}
