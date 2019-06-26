use crate::geometry::{Normal, Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct SurfaceContext {
    pub p : Point,
    pub n : Normal,
    pub u : f32,
    pub v : f32,
    pub dpdu : Vector,
    pub dpdv : Vector,
    pub dndu : Normal,
    pub dndv : Normal,
}

impl SurfaceContext {
    pub fn new(p : Point, n : Normal, (u, v) : (f32, f32), (dpdu, dpdv) : (Vector, Vector), (dndu, dndv) : (Normal, Normal)) -> SurfaceContext {
        SurfaceContext {
            p: p,
            n: n,
            u: u,
            v: v,
            dpdu: dpdu,
            dpdv: dpdv,
            dndu: dndu,
            dndv: dndv
        }
    }
}
