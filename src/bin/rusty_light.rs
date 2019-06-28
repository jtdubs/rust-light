use std::sync::Arc;
use std::f32::consts::*;

use clap::*;

use light::cameras::{Camera, PerspectiveCamera, OrthographicCamera, HemisphereCamera, SphereCamera, PerspectiveLensCamera};
use light::film::Film;
use light::filters::{BoxFilter, GaussianFilter, CachingFilter};
use light::scene::Scene;
use light::shapes::{Sphere, Disc, Cylinder, Paraboloid, Plane, Cone};
use light::renderer::{render, RendererSetup};
use light::geometry::{Vector, Trans};
use light::sampler::{SamplerFactory2D, Sampler2D, CentersSampler2D, LHCSampler2D};


struct SamplerFactory {
    n : usize,
}

impl SamplerFactory {
    pub fn new(n : usize) -> SamplerFactory {
        SamplerFactory { n: n }
    }
}

impl SamplerFactory2D for SamplerFactory {
    fn get_sampler(&self) -> Box<dyn Sampler2D> {
        if self.n == 1 {
            Box::new(CentersSampler2D::new())
        } else {
            Box::new(LHCSampler2D::new(self.n))
            // let w = (self.n as f32).sqrt().round() as usize;
            // Box::new(StrataSampler2D::new(w, w))
        }
    }
}


fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Rusty Light")
        .version("0.1")
        .author("Justin Dubs <jtdubs@gmail.com>")
        .arg(Arg::with_name("resolution")
                .long("res")
                .value_name("PRESET")
                .takes_value(true)
                .possible_values(&["4k", "2k", "1080p", "720p", "VGA", "QVGA"])
                .default_value("1080p"))
        .arg(Arg::with_name("filter")
                .long("filter")
                .value_name("TYPE")
                .takes_value(true)
                .possible_values(&["box", "gaussian"])
                .default_value("gaussian"))
        .arg(Arg::with_name("camera")
                .long("camera")
                .value_name("TYPE")
                .takes_value(true)
                .possible_values(&["perspective", "ortho", "hemisphere", "sphere", "perspective-lens"])
                .default_value("perspective"))
        .arg(Arg::with_name("lens-radius")
                .long("lens-radius")
                .value_name("D")
                .takes_value(true)
                .default_value("1")
                .required_if("camera", "perspective-lens"))
        .arg(Arg::with_name("focal-distance")
                .long("focal-distance")
                .value_name("D")
                .takes_value(true)
                .default_value("6")
                .required_if("camera", "perspective-lens"))
        .arg(Arg::with_name("fov")
                .long("fov")
                .value_name("DEG")
                .takes_value(true)
                .default_value("60")
                .required_ifs(&[("camera", "perspective"), ("camera", "perspective-lens")]))
        .arg(Arg::with_name("scale")
                .long("scale")
                .value_name("S")
                .takes_value(true)
                .default_value("10")
                .required_if("camera", "ortho"))
        .arg(Arg::with_name("samples")
                .long("samples")
                .value_name("N")
                .takes_value(true)
                .default_value("16"))
        .arg(Arg::with_name("output")
                .long("output")
                .value_name("PNG")
                .takes_value(true)
                .default_value("out/test.png"))
}

fn get_renderer_setup() -> Option<RendererSetup> {
    match get_app().get_matches_safe() {
        Err(e) => {
            println!("{:}", e);
            None
        },
        Ok(matches) => {
            let film_size = match matches.value_of("resolution").unwrap() {
                "4k"    => Some((3840, 2160)),
                "2k"    => Some((1920, 1080)),
                "1080p" => Some((1920, 1080)),
                "720p"  => Some((1280, 720)),
                "VGA"   => Some((640, 480)),
                "QVGA"  => Some((320, 240)),
                _       => None
            }.unwrap();

            let film = Film::new(film_size.0, film_size.1);

            let filter = match matches.value_of("filter").unwrap() {
                "box"      => Some(CachingFilter::new(&BoxFilter::new(0.5f32, 0.5f32))),
                "gaussian" => Some(CachingFilter::new(&GaussianFilter::new(1.4f32, 1.4f32, 0.25f32))),
                _          => None,
            }.unwrap();

            let camera : Arc<dyn Camera> = match matches.value_of("camera").unwrap() {
                "perspective" => {
                    let fov = matches.value_of("fov").unwrap().parse::<f32>().unwrap() * PI / 180f32;
                    Arc::new(PerspectiveCamera::new(fov, film.width as f32 / film.height as f32))
                },
                "perspective-lens" => {
                    let fov = matches.value_of("fov").unwrap().parse::<f32>().unwrap() * PI / 180f32;
                    let lr = matches.value_of("lens-radius").unwrap().parse::<f32>().unwrap();
                    let fd = matches.value_of("focal-distance").unwrap().parse::<f32>().unwrap();
                    Arc::new(PerspectiveLensCamera::new(fov, film.width as f32 / film.height as f32, lr, fd))
                },
                "ortho" => {
                    let scale = matches.value_of("scale").unwrap().parse::<f32>().unwrap();
                    Arc::new(OrthographicCamera::new(scale, film.width as f32 / film.height as f32))
                },
                "hemisphere" => Arc::new(HemisphereCamera::new()),
                _            => Arc::new(SphereCamera::new()),
            };

            let sampler_factory = Arc::new(SamplerFactory::new(matches.value_of("samples").unwrap().parse::<usize>().unwrap()));
                    
            let output_filename = String::from(matches.value_of("output").unwrap());

            Some(RendererSetup::new(film, filter, camera, sampler_factory, output_filename))
        }
    }
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    for z in vec![7f32].into_iter() { // , 10f32, 20f32, 40f32].into_iter() {
        scene.add(Arc::new(Sphere::unit().translate(&Vector::new( -5f32, 0.8f32, z))));
        scene.add(Arc::new(Sphere::new_partial(0.5f32, (-0.3f32, 0.3f32), PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-5f32, -0.8f32, z))));

        scene.add(Arc::new(Cylinder::unit().translate(&Vector::new( -3f32, 0.8f32, z))));
        scene.add(Arc::new(Cylinder::new_partial(0.5f32, 1f32, PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-3f32, -0.8f32, z))));

        scene.add(Arc::new(Disc::new_annulus(0.1f32, 0.5f32).translate(&Vector::new(-1f32, 0.8f32, z))));
        scene.add(Arc::new(Disc::new_partial_annulus(0.1f32, 0.5f32, PI * 1.5f32).rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(-1f32, -0.8f32, z))));
        
        scene.add(Arc::new(Plane::unit().translate(&Vector::new(1f32, 0.8f32, z))));
        scene.add(Arc::new(Plane::unit().rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(1f32, -0.8f32, z))));

        scene.add(Arc::new(Cone::unit().rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(3f32, 0.3f32, z))));
        scene.add(Arc::new(Cone::new_partial(0.5f32, 1f32, 0.2f32, 0.8f32, PI * 1.5f32).rotate(PI, &Vector::unit_z()).rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(3f32, -1.3f32, z))));

        scene.add(Arc::new(Paraboloid::unit().rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(5f32, 0.3f32, z))));
        scene.add(Arc::new(Paraboloid::new_partial(0.5f32, 1f32, 0.2f32, 0.8f32, PI * 1.5f32).rotate(PI, &Vector::unit_z()).rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(5f32, -1.3f32, z))));
    }

    scene
}

fn main() {
    env_logger::init();

    if let Some(setup) = get_renderer_setup() {
        render(setup, build_scene());
    }
}
