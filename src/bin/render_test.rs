use std::path::Path;
use std::sync::Arc;
use std::f32::consts::*;

use light::cameras::perspective::PerspectiveCamera;
use light::film::Film;
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

    let mut film = Film::new(1920, 1080);

    let filter = Arc::new(CachingFilter::new(&GaussianFilter::new(1.4f32, 1.4f32, 0.25f32)));

    let camera = Arc::new(PerspectiveCamera::new(FRAC_PI_3, film.width as f32 / film.height as f32));

    let sampler = Arc::new(SamplerFactory::new(16));

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

    render(camera, &mut film, filter, sampler, scene);

    match film.save(&Path::new("out/test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}
