use std::path::Path;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

use crate::scene::Scene;
use crate::sampler::{SamplerFactory2D, Sampler2D};
use crate::film::Film;
use crate::filters::{Filter, CachingFilter};
use crate::cameras::Camera;
use crate::geometry::Point;

type Patch = (u32, u32, u32, u32);

pub struct RendererSetup {
    pub film            : Film,
    pub filter          : CachingFilter,
    pub camera          : Arc<dyn Camera>,
    pub sampler_factory : Arc<dyn SamplerFactory2D>,
    pub output_filename : String,
}

impl RendererSetup {
    pub fn new(film : Film, filter : CachingFilter, camera : Arc<dyn Camera>, sampler_factory : Arc<SamplerFactory2D>, output_filename : String) -> RendererSetup {
        RendererSetup {
            film:            film,
            filter:          filter,
            camera:          camera,
            sampler_factory: sampler_factory,
            output_filename: output_filename,
        }
    }
}

pub fn render(setup : RendererSetup, scene : Scene) {
    let patches = get_patches(&setup.film, 16);

    let scene = Arc::new(scene);
    let filter = Arc::new(setup.filter);
    let film = Arc::new(Mutex::new(setup.film));

    let pool = ThreadPool::new(4);

    for patch in patches {
        let camera = setup.camera.clone();
        let filter = filter.clone();
        let scene = scene.clone();
        let film = film.clone();
        let sampler = setup.sampler_factory.get_sampler();
        pool.execute(move || { render_patch(patch, film, camera, filter, scene, sampler); });
    }

    pool.join();

    match film.lock().unwrap().save(&Path::new(&setup.output_filename)) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    };
}

pub fn render_patch(patch : Patch, film : Arc<Mutex<Film>>, camera : Arc<dyn Camera>, filter : Arc<CachingFilter>, scene : Arc<Scene>, mut sampler : Box<dyn Sampler2D>) {
    let (xs, ys, xe, ye) = patch;

    let the_film = film.lock().unwrap();
    let x_scale = 2f32 / (the_film.width as f32);
    let y_scale = 2f32 / (the_film.height as f32);
    drop(the_film);

    for x in xs..xe {
        for y in ys..ye {
            let mut sum = 0f32;
            let mut weight_sum = 0f32;

            for (dx, dy) in sampler.get_samples().into_iter() {
                let fx = (x as f32) + dx;
                let fy = (y as f32) + dy;
                let cx = fx * x_scale - 1f32;
                let cy = fy * y_scale - 1f32;
                let r = camera.cast(cx, cy);

                let v = match scene.intersect(&r) {
                    None => 0f32,
                    Some(i) => {
                        let fudge = ((Point::origin() - i.context.p.from(&i.shape)).to_normal().dot(&i.context.n.from(&i.shape).normalize()) / 2f32) + 0.5f32;
                        if ((i.context.u * 8f32).floor() as u32 % 2 == 0) ^ ((i.context.v * 8f32).floor() as u32 % 2 == 0) {
                            255f32 * (1f32 - fudge)
                        } else {
                            64f32 * (1f32 - fudge)
                        }
                    }
                };

                let w = filter.weight(dx - 0.5f32, dy - 0.5f32);
                sum += v * w;
                weight_sum += w;
            }

            film.lock().unwrap().splat(x, y, sum, weight_sum);
        }
    }
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
