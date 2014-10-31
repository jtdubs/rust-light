use scene::Scene;
use sampler::Sampler;
use aabb::AABB;

pub fn render(s : &mut Scene) {
    let mut bounds = AABB::new();
    for p in s.primitives.iter() {
        bounds.add_self_aabb(&p.world_bound());
    }

    let (min_z, max_z) = match bounds.range_z() {
        None => (0f64, 0f64),
        Some((n, x)) => (n, x),
    };

    let mut sampler = Sampler::new();
    let (fw, fh) = s.camera.get_film_size();
    for x in range(0u32, fw+1) {
        for y in range(0u32, fh+1) {
            for &(dx, dy) in sampler.lhc_2d(8).iter() {
                let fx = (x as f64) + dx;
                let fy = (y as f64) + dy;
                let r = s.camera.cast(fx, fy);
                match s.intersect(&r) {
                    None => s.camera.receive(fx, fy, 0u8),
                    Some(t) => {
                        let z = ((max_z - r.at_time(t).z) / (max_z - min_z)).min(1f64).max(0f64);
                        s.camera.receive(fx, fy, (z * 255f64) as u8)
                    }
                }
            }
        }
    }
}
