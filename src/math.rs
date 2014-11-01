extern crate std;

use std::iter::iterate;

pub fn quadratic(a : f32, b : f32, c : f32) -> Option<(f32, f32)> {
    let d = b*b - 4f32*a*c;
    if d < 0f32 {
        None
    } else {
        let q = if b < 0f32 { -(b - d.sqrt()) / 2f32 } else { -(b + d.sqrt()) / 2f32 };
        let r1 = q / a;
        let r2 = c / q;
        if r1 < r2 { Some((r1, r2)) } else { Some((r2, r1)) }
    }
}

pub fn radical_inverse(n : uint, b : uint) -> f32 {
    helper(0f32, n, 1f32 / (b as f32), 1f32 / (b as f32), b)
}

fn helper(r : f32, i : uint, inv_bi : f32, inv_base : f32, b : uint) -> f32 {
    if i == 0 {
        r
    } else {
        let di = i % b;
        helper(r + ((di as f32) * inv_bi), ((i as f32) * inv_base).trunc() as uint, inv_bi * inv_base, inv_base, b)
    }
}

pub fn van_der_corput(n : u32, scramble : u32) -> f32 {
    let num = ((reverse_bits(n) ^ scramble) >> 8) & 0xFFFFFF;
    let denom = 1u32 << 24;
    (num as f32) / (denom as f32)
}

pub fn sobol(n : u32, scramble : u32) -> f32 {
    let vs = std::iter::iterate(1u32 << 31, |x| { x ^ (x >> 1) }).take(32);
    let ns = range(0u, 32u).map(|s| { (n >> s) & 1 });
    let s = ns.zip(vs).map(|(a, b)| { a * b }).fold(scramble, |a, b| { a ^ b });
    (((s >> 8) & 0xFFFFFF) as f32) / ((1u32 << 24) as f32)
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
