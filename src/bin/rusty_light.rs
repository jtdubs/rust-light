use std::path::Path;
use std::sync::Arc;
use std::f32::consts::*;

use clap::{Arg,App};

use light::cameras::camera::Camera;
use light::cameras::perspective::PerspectiveCamera;
use light::cameras::orthographic::OrthographicCamera;
use light::cameras::hemisphere::HemisphereCamera;
use light::cameras::sphere::SphereCamera;
use light::film::Film;
use light::filters::box_filter::BoxFilter;
use light::filters::gaussian::GaussianFilter;
use light::filters::caching::CachingFilter;
use light::scene::Scene;
use light::shapes::disc::Disc;
use light::shapes::sphere::Sphere;
use light::shapes::paraboloid::Paraboloid;
use light::shapes::cone::Cone;
use light::shapes::plane::Plane;
use light::shapes::cylinder::Cylinder;
use light::renderer::render;
use light::geometry::vector::Vector;
use light::geometry::transform::Trans;
use light::sampler::*;


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
        }
    }
}


fn main() {
    env_logger::init();

    let safe_matches =
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
                 .possible_values(&["box", "guassian"])
                 .default_value("guassian"))
            .arg(Arg::with_name("camera")
                 .long("camera")
                 .value_name("TYPE")
                 .takes_value(true)
                 .possible_values(&["perspective", "ortho", "hemisphere", "sphere"])
                 .default_value("perspective"))
            .arg(Arg::with_name("fov")
                 .long("fov")
                 .value_name("DEG")
                 .takes_value(true)
                 .default_value("60")
                 .required_if("camera", "perspective"))
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
            .get_matches_safe();

    if safe_matches.is_err() {
        println!("{:}", safe_matches.unwrap_err());
        return;
    }

    let matches = safe_matches.unwrap();

    let (film_width, film_height) = match matches.value_of("resolution").unwrap() {
        "4k"    => Some((3840, 2160)),
        "2k"    => Some((1920, 1080)),
        "1080p" => Some((1920, 1080)),
        "720p"  => Some((1280, 720)),
        "VGA"   => Some((640, 480)),
        "QVGA"  => Some((320, 240)),
        _       => None
    }.unwrap();

    let mut film = Film::new(film_width, film_height);

    let filter = match matches.value_of("filter").unwrap() {
        "box"      => Some(Arc::new(CachingFilter::new(&BoxFilter::new(0.5f32, 0.5f32)))),
        "guassian" => Some(Arc::new(CachingFilter::new(&GaussianFilter::new(1.4f32, 1.4f32, 0.25f32)))),
        _          => None,
    }.unwrap();

    let camera : Arc<dyn Camera> = match matches.value_of("camera").unwrap() {
        "perspective" => {
            let fov = matches.value_of("fov").unwrap().parse::<f32>().unwrap() * PI / 180f32;
            Arc::new(PerspectiveCamera::new(fov, film.width as f32 / film.height as f32))
        },
        "ortho" => {
            let scale = matches.value_of("scale").unwrap().parse::<f32>().unwrap();
            Arc::new(OrthographicCamera::new(scale, film.width as f32 / film.height as f32))
        },
        "hemisphere" => Arc::new(HemisphereCamera::new()),
        _            => Arc::new(SphereCamera::new()),
    };

    let sampler_factory = Arc::new(SamplerFactory::new(matches.value_of("samples").unwrap().parse::<usize>().unwrap()));
            
    let out = matches.value_of("output").unwrap();

    let mut scene = Scene::new();

    scene.add(Arc::new(Sphere::unit().translate(&Vector::new( -5f32, 0.8f32, 7f32))));
    scene.add(Arc::new(Sphere::new_partial(0.5f32, (-0.3f32, 0.3f32), PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-5f32, -0.8f32, 7f32))));

    scene.add(Arc::new(Cylinder::unit().translate(&Vector::new( -3f32, 0.8f32, 7f32))));
    scene.add(Arc::new(Cylinder::new_partial(0.5f32, 1f32, PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-3f32, -0.8f32, 7f32))));

    scene.add(Arc::new(Disc::new_annulus(0.1f32, 0.5f32).translate(&Vector::new(-1f32, 0.8f32, 7f32))));
    scene.add(Arc::new(Disc::new_partial_annulus(0.1f32, 0.5f32, PI * 1.5f32).rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(-1f32, -0.8f32, 7f32))));
    
    scene.add(Arc::new(Plane::unit().translate(&Vector::new(1f32, 0.8f32, 7f32))));
    scene.add(Arc::new(Plane::unit().rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(1f32, -0.8f32, 7f32))));

    scene.add(Arc::new(Cone::unit().rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(3f32, 0.3f32, 7f32))));
    scene.add(Arc::new(Cone::new_partial(0.5f32, 1f32, 0.2f32, 0.8f32, PI * 1.5f32).rotate(PI, &Vector::unit_z()).rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(3f32, -1.3f32, 7f32))));

    scene.add(Arc::new(Paraboloid::unit().rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(5f32, 0.3f32, 7f32))));
    scene.add(Arc::new(Paraboloid::new_partial(0.5f32, 1f32, 0.2f32, 0.8f32, PI * 1.5f32).rotate(PI, &Vector::unit_z()).rotate(-FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(5f32, -1.3f32, 7f32))));

    render(camera, &mut film, filter, sampler_factory, scene);

    match film.save(&Path::new(out)) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    };
}
