use std::path::Path;
use std::sync::Arc;
use std::f32::consts::*;

use light::cameras::perspective::PerspectiveCamera;
use light::film::Film;
use light::filters::box_filter::BoxFilter;
use light::filters::gaussian::GaussianFilter;
use light::filters::caching::CachingFilter;
use light::scene::Scene;
// use light::shapes::rectangular_prism::RectangularPrism;
// use light::shapes::triangle::Triangle;
use light::shapes::disc::Disc;
use light::shapes::sphere::Sphere;
// use light::shapes::paraboloid::Paraboloid;
// use light::shapes::cone::Cone;
use light::shapes::plane::Plane;
use light::shapes::cylinder::Cylinder;
use light::renderer::render;
use light::geometry::vector::Vector;
use light::geometry::transform::Trans;

fn main() {
    env_logger::init();

    let mut film = Film::new(1920, 1080);

    let filter = CachingFilter::new(&GaussianFilter::new(1.4f32, 1.4f32, 0.25f32));
    // let filter = BoxFilter::new(0.5f32, 0.5f32);
    let camera = PerspectiveCamera::new(FRAC_PI_3, film.width as f32 / film.height as f32);

    let mut scene = Scene::new();

    scene.add(Arc::new(Sphere::unit().translate(&Vector::new( -3f32, 0.8f32, 4f32))));
    scene.add(Arc::new(Sphere::new_partial(0.5f32, (-0.3f32, 0.3f32), PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-3f32, -0.8f32, 4f32))));

    scene.add(Arc::new(Cylinder::unit().translate(&Vector::new( -1f32, 0.8f32, 4f32))));
    scene.add(Arc::new(Cylinder::new_partial(0.5f32, 1f32, PI).rotate(FRAC_PI_2, &Vector::unit_x()).translate(&Vector::new(-1f32, -0.8f32, 4f32))));

    scene.add(Arc::new(Disc::new_annulus(0.1f32, 0.5f32).translate(&Vector::new(1f32, 0.8f32, 4f32))));
    scene.add(Arc::new(Disc::new_partial_annulus(0.1f32, 0.5f32, PI * 1.5f32).rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(1f32, -0.8f32, 4f32))));
    
    scene.add(Arc::new(Plane::unit().translate(&Vector::new(3f32, 0.8f32, 4f32))));
    scene.add(Arc::new(Plane::unit().rotate(FRAC_PI_3, &Vector::unit_x()).translate(&Vector::new(3f32, -0.8f32, 4f32))));

    // scene.add(Arc::new(Triangle::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(-4f32, 3f32, 10f32))));
    // scene.add(Arc::new(RectangularPrism::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-6f32, -3f32, 10f32))));
    // scene.add(Arc::new(Sphere::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-3f32, -3f32, 10f32))));
    // scene.add(Arc::new(Cylinder::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(0f32, -3f32, 10f32))));
    // scene.add(Arc::new(Paraboloid::unit().rotate3(-pi_2, 0f32, 0f32).translate(&Vector::new(3f32, -3f32, 10f32))));
    // scene.add(Arc::new(Cone::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(6f32, -3f32, 10f32))));

    render(camera, &mut film, filter, scene);

    match film.save(&Path::new("out/test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}
