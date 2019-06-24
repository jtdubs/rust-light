use log::*;
use threadpool::ThreadPool;
use std::sync::mpsc::{channel,Sender};
use std::sync::Arc;

use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::film::Film;
use crate::filters::filter::Filter;
use crate::cameras::camera::Camera;
use crate::geometry::point::Point;

type Patch = (u32, u32, u32, u32);
type Splat = (u32, u32, f32, f32);
type Splats = Box<Vec<Splat>>;

pub fn render(camera : impl Camera + 'static, film : &mut Film, filter : impl Filter + 'static, scene : Scene) {
    let camera = Arc::new(camera);
    let scene = Arc::new(scene);
    let filter = Arc::new(filter);

    let fw = film.width;
    let fh = film.height;

    let pool = ThreadPool::new(3);
    let (tx, rx) = channel::<Splats>();

    for patch in get_patches(film, 16) {
        let tx = tx.clone();
        let camera = camera.clone();
        let filter = filter.clone();
        let scene = scene.clone();
        pool.execute(move || { render_patch(patch, tx, camera, filter, scene, fw, fh); });
    }

    drop(tx);

    for v in rx {
        for (x, y, s, w) in v.into_iter() {
            film.splat(x, y, s, w);
        }
    }
}

pub fn render_patch(patch : Patch, tx : Sender<Splats>, camera : Arc<impl Camera>, filter : Arc<impl Filter>, scene : Arc<Scene>, film_width : u32, film_height : u32) {
    debug!("render_patch({:?})", patch);

    let mut sampler = Sampler::new();

    let (xs, ys, xe, ye) = patch;

    let x_scale = 2f32 / (film_width as f32);
    let y_scale = 2f32 / (film_height as f32);

    let (ex, ey) = filter.extent();

    let mut film_updates = Box::new(Vec::with_capacity(51200));
    for x in xs..xe {
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
                        let fudge = ((Point::origin() - i.context.p).to_normal().dot(&i.context.n.normalize()) / 2f32) + 0.5f32;
                        if ((i.context.u * 8f32).floor() as u32 % 2 == 0) ^ ((i.context.v * 8f32).floor() as u32 % 2 == 0) {
                            255f32 * (1f32 - fudge)
                        } else {
                            64f32 * (1f32 - fudge)
                        }
                    }
                };

                let min_x = (fx - 0.5f32 - ex).ceil().max(0f32) as u32;
                let min_y = (fy - 0.5f32 - ey).ceil().max(0f32) as u32;
                let max_x = (fx + 0.5f32 + ex).min(film_width as f32 - 1f32) as u32;
                let max_y = (fy + 0.5f32 + ey).min(film_height as f32 - 1f32) as u32;
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
    }
    tx.send(film_updates).unwrap();

    drop(tx);
}

pub fn get_patches(film : &Film, patch_size : u32) -> Vec<Patch> {
    let fw = film.width;
    let fh = film.height;

    let patch_width  = patch_size;
    let patch_height = patch_size;

    let n = fw / patch_width;
    let m = fh / patch_height;

    let mut patches = Vec::with_capacity((n * m) as usize);
    for x in 0..n {
        for y in 0..m {
            let xs = x * patch_width;
            let ys = y * patch_height;
            let mut xe = (x+1) * patch_width;
            let mut ye = (y+1) * patch_height;

            if x == (n-1) && fw % patch_width != 0 {
                xe = fw;
            }

            if y == (m-1) && fh % patch_height != 0 {
                ye = fh;
            }

            patches.push((xs, ys, xe, ye));
        }
    }

    patches
}
