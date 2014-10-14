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

pub fn radical_inverse(n : i64, b : i64) -> f64 {
    0f64
}
