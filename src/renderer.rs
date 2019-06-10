use scene::Scene;
use sampler::Sampler;
use film::Film;
use cameras::camera::Camera;

pub fn render(camera : &Camera, film : &mut Film, scene : &mut Scene) {
    let (min_z, max_z) = match scene.bounds().range_z() {
        None => (0f32, 0f32),
        Some((n, x)) => (n, x),
    };

    let depth = max_z - min_z;

    let mut sampler = Sampler::new();
    let fw = film.width as f32;
    let fh = film.height as f32;
    let x_scale = 2f32 / fw;
    let y_scale = 2f32 / fh;

    for x in 0f32..fw {
        for y in 0f32..fh {
            for &(dx, dy) in sampler.lhc_2d(8).iter() {
                let fx = x + dx;
                let fy = y + dy;
                let cx = fx * x_scale - 1f32;
                let cy = fy * y_scale - 1f32;
                let r = camera.cast(cx, cy);
                match scene.intersect(&r) {
                    None => film.add_sample(fx, fy, 0u8),
                    Some(i) => {
                        let z = (max_z - i.point.z) / depth;
                        film.add_sample(fx, fy, (z * 255f32) as u8)
                    }
                }
            }
        }
    }
}

