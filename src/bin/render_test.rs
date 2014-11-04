extern crate light;

use light::cameras::perspective::PerspectiveCamera;
use light::film::Film;
use light::filter::Filter;
use light::scene::Scene;
use light::shapes::sphere::Sphere;
use light::shapes::cylinder::Cylinder;
use light::shapes::disc::Disc;
use light::shapes::rectangular_prism::RectangularPrism;
use light::shapes::plane::Plane;
use light::shapes::triangle::Triangle;
use light::shapes::cone::Cone;
use light::shapes::paraboloid::Paraboloid;
use light::primitive::Primitive;
use light::renderer::render;
use light::geometry::vector::Vector;
use light::geometry::transform::Trans;

fn main() {
    let pi_4 : f32 = Float::frac_pi_4();
    let pi_2 : f32 = Float::frac_pi_2();
    let pi_3 : f32 = Float::frac_pi_3();

    let ref mut film = Film::new(1280u32, 720u32, Filter::new_box(1f32, 1f32));
    let ref camera = PerspectiveCamera::new(pi_3, (film.width as f32 / film.height as f32));

    let mut scene = Scene::new();

    scene.add(Primitive::new(box Triangle::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(-4f32, 3f32, 10f32))));
    scene.add(Primitive::new(box Plane::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(0f32, 3f32, 10f32))));
    scene.add(Primitive::new(box Disc::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(4f32, 3f32, 10f32))));
    scene.add(Primitive::new(box RectangularPrism::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-6f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Sphere::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-3f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Cylinder::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(0f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Paraboloid::unit().rotate3(-pi_2, 0f32, 0f32).translate(&Vector::new(3f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Cone::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(6f32, -3f32, 10f32))));

    render(camera, film, &mut scene);

    match film.save(&Path::new("test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}

