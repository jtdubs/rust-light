extern crate light;

use light::camera::PerspectiveCamera;
use light::film::Film;
use light::filter::Filter;
use light::scene::Scene;
use light::sphere::Sphere;
use light::cylinder::Cylinder;
use light::disc::Disc;
use light::rectangular_prism::RectangularPrism;
use light::plane::Plane;
use light::triangle::Triangle;
use light::cone::Cone;
use light::paraboloid::Paraboloid;
use light::primitive::Primitive;
use light::renderer::render;
use light::vector::Vector;
use light::transform::Trans;

fn main() {
    let pi_4 : f32 = Float::frac_pi_4();
    let pi_2 : f32 = Float::frac_pi_2();

    let mut scene = 
        Scene::new(
            box PerspectiveCamera::new(
                box Film::new(1280u32, 720u32,
//                    Filter::new_gaussian(2f32, 2f32, 0.25f32)), 
                    Filter::new_box(1f32, 1f32)), 
                Float::frac_pi_3()));

    scene.add(Primitive::new(box Triangle::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(-4f32, 3f32, 10f32))));
    scene.add(Primitive::new(box Plane::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(0f32, 3f32, 10f32))));
    scene.add(Primitive::new(box Disc::unit().rotate3(-pi_4, 0f32, 0f32).translate(&Vector::new(4f32, 3f32, 10f32))));
    scene.add(Primitive::new(box RectangularPrism::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-6f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Sphere::unit().rotate3(0f32, 0f32, 0f32).translate(&Vector::new(-3f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Cylinder::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(0f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Paraboloid::unit().rotate3(-pi_2, 0f32, 0f32).translate(&Vector::new(3f32, -3f32, 10f32))));
    scene.add(Primitive::new(box Cone::unit().rotate3(pi_2, 0f32, 0f32).translate(&Vector::new(6f32, -3f32, 10f32))));

    render(&mut scene);

    match scene.camera.save(&Path::new("test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}

