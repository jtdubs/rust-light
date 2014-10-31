extern crate light;

use light::camera::Camera;
use light::film::Film;
use light::filter::Filter;
use light::scene::Scene;
use light::shape::Shape;
use light::primitive::Primitive;
use light::renderer::render;
use light::vector::Vector;
use light::transform::Trans;

fn main() {
    let pi_4 : f64 = Float::frac_pi_4();
    let pi_2 : f64 = Float::frac_pi_2();

    let mut scene = 
        Scene::new(
            Camera::new_perspective(
                box Film::new(512u32, 256u32,
                    Filter::new_gaussian(2f64, 2f64, 0.25f64)), 
                Float::frac_pi_3()));

    scene.add(Primitive::new(Shape::new_unit_triangle().rotate3(-pi_4, 0f64, 0f64).translate(&Vector::new(-4f64, 3f64, 10f64))));
    scene.add(Primitive::new(Shape::new_unit_plane().rotate3(-pi_4, 0f64, 0f64).translate(&Vector::new(0f64, 3f64, 10f64))));
    scene.add(Primitive::new(Shape::new_unit_disc().rotate3(-pi_4, 0f64, 0f64).translate(&Vector::new(4f64, 3f64, 10f64))));
    scene.add(Primitive::new(Shape::new_unit_box().rotate3(0f64, 0f64, 0f64).translate(&Vector::new(-6f64, -3f64, 10f64))));
    scene.add(Primitive::new(Shape::new_unit_sphere().rotate3(0f64, 0f64, 0f64).translate(&Vector::new(-3f64, -3f64, 10f64))));
    // scene.add(Primitive::new(Shape::new_unit_cylinder().rotate3(pi_2, 0f64, 0f64).translate(&Vector::new(0f64, -3f64, 10f64))));
    scene.add(Primitive::new(Shape::new_unit_paraboloid().rotate3(-pi_2, 0f64, 0f64).translate(&Vector::new(3f64, -3f64, 10f64))));
    // scene.add(Primitive::new(Shape::new_unit_cone().rotate3(pi_2, 0f64, 0f64).translate(&Vector::new(6f64, -3f64, 10f64))));

    render(&mut scene);

    match scene.camera.save(&Path::new("test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}

