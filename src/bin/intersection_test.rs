use log::*;

use light::shapes::sphere::Sphere;
use light::shapes::shape::{Shape,Intersection};
use light::geometry::vector::Vector;
use light::geometry::ray::Ray;
use light::geometry::point::Point;
use light::geometry::transform::Trans;

fn main() {
    env_logger::init();

    let sphere = Sphere::unit().translate(&Vector::new(0f32, 0f32, 0f32));

    let origins = [
        Point::new(-5f32,  0f32,  0f32), 
        Point::new( 5f32,  0f32,  0f32), 
        Point::new( 0f32, -5f32,  0f32), 
        Point::new( 0f32,  5f32,  0f32), 
        Point::new( 0f32,  0f32,  5f32), 
        Point::new( 0f32,  0f32, -5f32)
    ];

    let ref hits : Vec<Intersection> = origins.iter().map(|o| {
        let d = Point::origin() - *o;
        let ray = Ray::new(&o, &d);
        sphere.intersect(&ray).unwrap()
    }).collect();

    println!("clf;");
    println!("figure (1)");
    println!("[x, y, z] = sphere (40);");
    println!("surf (x, y, z);");
    println!("hold on;");
    println!("grid off;");
    println!("box off;");
    println!("axis ([{}, {}, {}, {}, {}, {}], \"square\");", -5f32, 5f32, -5f32, 5f32, -5f32, 5f32);
    println!("daspect ([1, 1, 1]);");
    println!("pbaspect ([1, 1, 1]);");
    println!("title (\"{}\");", "sphere intersections");

    print!("ox = [");
    for h in hits {
        print!("{}, ", h.ray.origin.x);
    }
    println!("];");

    print!("oy = [");
    for h in hits {
        print!("{}, ", h.ray.origin.y);
    }
    println!("];");

    print!("oz = [");
    for h in hits {
        print!("{}, ", h.ray.origin.z);
    }
    println!("];");

    print!("dx = [");
    for h in hits {
        print!("{}, ", h.ray.direction.x);
    }
    println!("];");

    print!("dy = [");
    for h in hits {
        print!("{}, ", h.ray.direction.y);
    }
    println!("];");

    print!("dz = [");
    for h in hits {
        print!("{}, ", h.ray.direction.z);
    }
    println!("];");

    println!("q = quiver3(ox, oy, oz, dx, dy, dz, 0);");
    println!("set (q, \"maxheadsize\", 0);");
    println!("hold off;");
}
