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
    let mut scene = 
        Scene::new(
            Camera::new_perspective(
                box Film::new(256u32, 256u32,
                    Filter::new_gaussian(2f64, 2f64, 0.25f64)), 
                Float::frac_pi_3()));

    scene.add(Primitive::new(Shape::new_unit_box().translate(&Vector::new(1f64, 1f64, -4f64))));

    render(&mut scene);

    match scene.camera.save(&Path::new("test.png")) {
        Ok(_) => { },
        Err(m) => println!("{}", m),
    }
}

