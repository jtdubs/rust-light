use std::path::Path;
use threadpool::{ThreadPool,Builder};
use std::sync::mpsc::{sync_channel,channel};
use std::sync::Arc;

use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::film::Film;
use crate::filters::filter::Filter;
use crate::cameras::camera::Camera;

pub fn render<F : Filter, C : Camera + 'static>(camera : C, film : &mut Film, filter : F, scene : Scene) {
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

    let (ex, ey) = filter.extent();

    for x in 0..fw {
        for y in 0..fh {
            for (dx, dy) in sampler.lhc_2d(8).into_iter() {
                let fx = (x as f32) + dx;
                let fy = (y as f32) + dy;
                let cx = fx * x_scale - 1f32;
                let cy = fy * y_scale - 1f32;
                let r = camera.cast(cx, cy);

                let v = match scene.intersect(&r) {
                    None => 0f32,
                    Some(i) => {
                        let z = (max_z - i.point.z) / depth;
                        z * 255f32
                    }
                };

                let min_x = (fx - 0.5f32 - ex).ceil().max(0f32) as u32;
                let min_y = (fy - 0.5f32 - ey).ceil().max(0f32) as u32;
                let max_x = (fx + 0.5f32 + ex).min(fw as f32 - 1f32) as u32;
                let max_y = (fy + 0.5f32 + ey).min(fh as f32 - 1f32) as u32;
                let ox = 0.5f32 - fx;
                let oy = 0.5f32 - fy;

                for ux in min_x..=max_x {
                    for uy in min_y..=max_y {
                        let w = filter.weight(ux as f32 + ox, uy as f32 + oy);
                        film.splat(ux, uy, v * w, w);
                    }
                }
            }
        }
    }
}


pub fn renderp<F : Filter + 'static, C : Camera + 'static>(camera : C, film : &mut Film, filter : F, scene : Scene) {
    let (min_z, max_z) = match scene.bounds.range_z() {
        None => (0f32, 0f32),
        Some((n, x)) => (n, x),
    };

    let depth = max_z - min_z;

    let fw = film.width;
    let fh = film.height;
    let x_scale = 2f32 / (fw as f32);
    let y_scale = 2f32 / (fh as f32);

    let patch_width  = fw / 16;
    let patch_height = fh / 16;

    let mut patches : Vec<(u32, u32, u32, u32)> = Vec::new();
    for x in 0..16 {
        for y in 0..16 {
            patches.push((x * patch_width, y * patch_height, (x+1) * patch_width, (y+1) * patch_height));
        }
    }

    let (tx, rx) = channel::<Box<Vec<(u32, u32, f32, f32)>>>();

    let camera = Arc::new(camera);
    let scene = Arc::new(scene);
    let filter = Arc::new(filter);

    let pool = ThreadPool::new(3);
    // let pool = Builder::new().build();

    for (xs, ys, xe, ye) in patches {
        let tx = tx.clone();

        let width = film.width;
        let height = film.height;

        let camera = camera.clone();
        let scene = scene.clone();
        let filter = filter.clone();

        let (ex, ey) = filter.extent();

        pool.execute(move || {
            let mut sampler = Sampler::new();

            for x in xs..xe {
                let mut film_updates = Box::new(Vec::with_capacity(16384));
                for y in ys..ye {
                    for (dx, dy) in sampler.lhc_2d(8).into_iter() {
                        let fx = (x as f32) + dx;
                        let fy = (y as f32) + dy;
                        let cx = fx * x_scale - 1f32;
                        let cy = fy * y_scale - 1f32;
                        let r = camera.cast(cx, cy);

                        let v = match scene.intersect(&r) {
                            None => 0f32,
                            Some(i) => {
                                let z = (max_z - i.point.z) / depth;
                                z * 255f32
                            }
                        };

                        let min_x = (fx - 0.5f32 - ex).ceil().max(0f32) as u32;
                        let min_y = (fy - 0.5f32 - ey).ceil().max(0f32) as u32;
                        let max_x = (fx + 0.5f32 + ex).min(width as f32 - 1f32) as u32;
                        let max_y = (fy + 0.5f32 + ey).min(height as f32 - 1f32) as u32;
                        let ox = 0.5f32 - fx;
                        let oy = 0.5f32 - fy;

                        for ux in min_x..=max_x {
                            for uy in min_y..=max_y {
                                let w = filter.weight(ux as f32 + ox, uy as f32 + oy);
                                film_updates.push((ux, uy, v * w, w));
                            }
                        }
                    }
                }
                tx.send(film_updates);
            }

            drop(tx);
        });
    }

    drop(tx);

    for v in rx {
        for (x, y, s, w) in v.into_iter() {
            film.splat(x, y, s, w);
        }
    }

    match film.save(&Path::new("out/test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}
