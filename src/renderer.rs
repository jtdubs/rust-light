use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::film::Film;
use crate::filters::filter::Filter;
use crate::cameras::camera::Camera;

pub fn render<F : Filter>(camera : &dyn Camera, film : &mut Film<F>, scene : &mut Scene) {
    let (min_z, max_z) = match scene.bounds.range_z() {
        None => (0f32, 0f32),
        Some((n, x)) => (n, x),
    };

    let depth = max_z - min_z;

    let mut sampler = Sampler::new();
    let fw = film.width;
    let fh = film.height;
    let x_scale = 2f32 / (fw as f32);
    let y_scale = 2f32 / (fh as f32);

    for x in 0..fw {
        for y in 0..fh {
            for (dx, dy) in sampler.lhc_2d(8).into_iter() {
                let fx = (x as f32) + dx;
                let fy = (y as f32) + dy;
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

