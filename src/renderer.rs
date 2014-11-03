use scene::Scene;
use sampler::Sampler;
use film::Film;
use camera::Camera;

pub fn render(camera : &Camera, film : &mut Film, scene : &mut Scene) {
    let (min_z, max_z) = match scene.bounds().range_z() {
        None => (0f32, 0f32),
        Some((n, x)) => (n, x),
    };

    let mut sampler = Sampler::new();
    let fw = film.width as f32;
    let fh = film.height as f32;

    for x in range(0u32, film.width+1) {
        for y in range(0u32, film.height+1) {
            for &(dx, dy) in sampler.lhc_2d(8).iter() {
                let fx = (x as f32) + dx;
                let fy = (y as f32) + dy;
                let cx = (fx / fw) * 2f32 - 1f32;
                let cy = (fy / fh) * 2f32 - 1f32;
                let r = camera.cast(cx, cy);
                match scene.intersect(&r) {
                    None => film.add_sample(fx, fy, 0u8),
                    Some(t) => {
                        let z = ((max_z - r.at_time(t).z) / (max_z - min_z)).min(1f32).max(0f32);
                        film.add_sample(fx, fy, (z * 255f32) as u8)
                    }
                }
            }
        }
    }
}

