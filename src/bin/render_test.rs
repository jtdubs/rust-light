extern crate light;

use std::path::Path;

use light::cameras::perspective::PerspectiveCamera;
use light::film::Film;
use light::filters::gaussian::GaussianFilter;
use light::filters::caching::CachingFilter;
use light::scene::Scene;
use light::shapes::rectangular_prism::RectangularPrism;
use light::shapes::triangle::Triangle;
use light::shapes::disc::Disc;
use light::shapes::sphere::Sphere;
use light::shapes::paraboloid::Paraboloid;
use light::shapes::cone::Cone;
use light::shapes::plane::Plane;
use light::shapes::cylinder::Cylinder;
use light::renderer::render;
use light::geometry::vector::Vector;
use light::geometry::transform::Trans;

fn main() {
    let pi_4 : f32 = std::f32::consts::FRAC_PI_4;
    let pi_2 : f32 = std::f32::consts::FRAC_PI_2;
    let pi_3 : f32 = std::f32::consts::FRAC_PI_3;

    // let ref mut film = Film::new(320, 240, Box::new(CachingFilter::new(&GaussianFilter::new(2f32, 2f32, 0.25f32))));
    // let ref mut film = Film::new(640, 480, Box::new(CachingFilter::new(&GaussianFilter::new(2f32, 2f32, 0.25f32))));
    let ref mut film = Film::new(1280, 720, CachingFilter::new(&GaussianFilter::new(2f32, 2f32, 0.25f32)));
    // let ref mut film = Film::new(1920, 1080, CachingFilter::new(&GaussianFilter::new(2f32, 2f32, 0.25f32)));
    let ref camera = PerspectiveCamera::new(pi_3, film.width as f32 / film.height as f32);

    let mut scene = Scene::new();

    scene.add(Triangle::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(-4f32, 3f32, 10f32)));
    scene.add(Plane::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(0f32, 3f32, 10f32)));
    scene.add(Disc::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(4f32, 3f32, 10f32)));
    scene.add(RectangularPrism::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-6f32, -3f32, 10f32)));
    scene.add(Sphere::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-3f32, -3f32, 10f32)));
    scene.add(Cylinder::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(0f32, -3f32, 10f32)));
    scene.add(Paraboloid::unit().rotate3(-pi_2, 0f32, 0f32).translate(&Vector::new(3f32, -3f32, 10f32)));
    scene.add(Cone::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(6f32, -3f32, 10f32)));

    render(camera, film, &mut scene);

    match film.save(&Path::new("out/test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}

