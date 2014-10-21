extern crate std;

use std::iter::iterate;

pub fn quadratic(a : f64, b : f64, c : f64) -> Option<[f64, ..2]> {
    let d = b*b - 4f64*a*c;
    if d < 0f64 {
        None
    } else {
        let q = if b < 0f64 { -(b - d.sqrt()) / 2f64 } else { -(b + d.sqrt()) / 2f64 };
        let r1 = q / a;
        let r2 = c / q;
        if r1 < r2 { Some([r1, r2]) } else { Some([r2, r1]) }
    }
}

pub fn radical_inverse(n : uint, b : uint) -> f64 {
    helper(0f64, n, 1f64 / (b as f64), 1f64 / (b as f64), b)
}

fn helper(r : f64, i : uint, inv_bi : f64, inv_base : f64, b : uint) -> f64 {
    if i == 0 {
        r
    } else {
        let di = i % b;
        helper(r + ((di as f64) * inv_bi), ((i as f64) * inv_base).trunc() as uint, inv_bi * inv_base, inv_base, b)
    }
}

pub fn van_der_corput(n : u32, scramble : u32) -> f64 {
    let num = ((reverse_bits(n) ^ scramble) >> 8) & 0xFFFFFF;
    let denom = 1u32 << 24;
    (num as f64) / (denom as f64)
}

pub fn sobol(n : u32, scramble : u32) -> f64 {
    let vs = std::iter::iterate(1u32 << 31, |x| { x ^ (x >> 1) }).take(32);
    let ns = range(0u, 32u).map(|s| { (n >> s) & 1 });
    let s = ns.zip(vs).map(|(a, b)| { a * b }).fold(scramble, |a, b| { a ^ b });
    (((s >> 8) & 0xFFFFFF) as f64) / ((1u32 << 24) as f64)
}

pub fn reverse_bits(n : u32) -> u32 {
    let mut r = n;
    r = (r << 16) | (r >> 16);
    r = (r & 0x00FF00FF) >> 8 | (r & 0xFF00FF00) << 8;
    r = (r & 0x0F0F0F0F) >> 4 | (r & 0xF0F0F0F0) << 4;
    r = (r & 0x33333333) >> 2 | (r & 0xCCCCCCCC) << 2;
    r = (r & 0x55555555) >> 1 | (r & 0xAAAAAAAA) << 1;
    r
}
