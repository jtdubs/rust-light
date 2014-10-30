use scene::Scene;
use sampler::Sampler;

pub fn render(s : &mut Scene) {
    let mut sampler = Sampler::new();
    let (fw, fh) = s.camera.get_film_size();
    for x in range(0u32, fw+1) {
        for y in range(0u32, fh+1) {
            for &(dx, dy) in sampler.s02_2d(x, y, 8).iter() {
                let fx = (x as f64) + dx;
                let fy = (y as f64) + dy;
                let r = s.camera.cast(fx, fy);
                match s.intersect(&r) {
                    None => s.camera.receive(fx, fy, 0u8),
                    Some(_) => s.camera.receive(fx, fy, 255u8),
                }
            }
        }
    }
}

