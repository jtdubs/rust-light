use crate::shapes::SurfaceContext;

#[derive(Copy, Clone, Debug)]
pub struct TextureContext {
    pub s : f32,
    pub t : f32,
    pub dsdx : f32,
    pub dsdy : f32,
    pub dtdx : f32,
    pub dtdy : f32,
}

impl TextureContext {
    pub fn new(s : f32, t : f32, dsdx : f32, dsdy : f32, dtdx : f32, dtdy : f32) -> TextureContext {
        TextureContext { s: s, t: t, dsdx: dsdx, dsdy: dsdy, dtdx: dtdx, dtdy: dtdy }
    }
}

pub trait TextureMapping2D {
    fn map(&self, s : SurfaceContext) -> TextureContext;
}

#[derive(Copy, Clone, Debug)]
pub struct UVMapping2D {
    pub su : f32,
    pub sv : f32,
    pub du : f32,
    pub dv : f32,
}

impl UVMapping2D {
    pub fn new(su : f32, sv : f32, du : f32, dv : f32) -> UVMapping2D {
        UVMapping2D { su: su, sv: sv, du: du, dv: dv }
    }
}

impl TextureMapping2D for UVMapping2D {
    fn map(&self, s : SurfaceContext) -> TextureContext {
        TextureContext {
            s: self.su * s.u + self.du,
            t: self.sv * s.v + self.dv,
            dsdx: 0f32,
            dsdy: 0f32,
            dtdx: 0f32,
            dtdy: 0f32,
        }
    }
}
