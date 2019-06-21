use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use scoped_threadpool::Pool;

use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::film::Film;
use crate::filters::filter::Filter;
use crate::cameras::camera::Camera;

pub fn render<F : Filter, C : Camera + 'static>(camera : C, film : &mut Film<F>, scene : Scene) {
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


pub fn renderp<F : Filter, C : Camera + 'static>(camera : C, film : &mut Film<F>, scene : Scene) {
    let (min_z, max_z) = match scene.bounds.range_z() {
        None => (0f32, 0f32),
        Some((n, x)) => (n, x),
    };

    let depth = max_z - min_z;

    let fw = film.width;
    let fh = film.height;
    let x_scale = 2f32 / (fw as f32);
    let y_scale = 2f32 / (fh as f32);

    let patch_width  = fw / 2;
    let patch_height = fh / 2;

    let mut patches : Vec<(u32, u32, u32, u32)> = Vec::new();
    for x in 0..2 {
        for y in 0..2 {
            patches.push((x * patch_width, y * patch_height, (x+1) * patch_width, (y+1) * patch_height));
        }
    }

    let (tx, rx) = channel::<(f32, f32, u8)>();

    let camera = Arc::new(camera);
    let scene = Arc::new(scene);

    for p in patches {
        let tx = tx.clone();
        let (xs, ys, xe, ye) = p.clone();

        let camera = camera.clone();
        let scene = scene.clone();

        thread::spawn(move || {
            let mut sampler = Sampler::new();

            for x in xs..xe {
                for y in ys..ye {
                    for (dx, dy) in sampler.lhc_2d(8).into_iter() {
                        let fx = (x as f32) + dx;
                        let fy = (y as f32) + dy;
                        let cx = fx * x_scale - 1f32;
                        let cy = fy * y_scale - 1f32;
                        let r = camera.cast(cx, cy);
                        match scene.intersect(&r) {
                            None => {
                                tx.send((fx, fy, 0u8));
                            }
                            Some(i) => {
                                let z = (max_z - i.point.z) / depth;
                                tx.send((fx, fy, (z * 255f32) as u8));
                            }
                        }
                    }
                }
            }
            drop(tx);
        });
    };

    drop(tx);

    for (x, y, s) in rx {
        film.add_sample(x, y, s);
    }
}
