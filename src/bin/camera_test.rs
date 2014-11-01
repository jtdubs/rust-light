extern crate light;

use std::num::FloatMath;
use light::camera::{Camera,PerspectiveCamera,OrthoCamera};
use light::film::Film;
use light::filter::Filter;
use light::ray::Ray;

fn get_rays(c : &Camera) -> Vec<Ray> {
    let (fw, fh) = c.get_film_size();
    let mut res = Vec::with_capacity((fw * fh) as uint);
    for x in range(0, fw) {
        for y in range(0, fh) {
            let r = c.cast((x as f32) + 0.5f32, (y as f32) + 0.5f32);
            res.push(r);
        }
    }
    res
}

fn make_film() -> Film {
    Film::new(16, 12, Filter::new_box(1f32, 1f32))
}

fn main() {
    println!("clf;");
    draw(1, "Perspective (60)", &Camera::new_perspective(box make_film(), Float::frac_pi_3()));
    draw(2, "Perspective (90)", &Camera::new_perspective(box make_film(), Float::frac_pi_2()));
    draw(3, "Orthographic", &Camera::new_ortho(box make_film(), 1f32));
}

fn draw(ix : int, title : &str, c : &Camera) {
    let (ifw, ifh) = c.get_film_size();
    let fw = ifw as f32;
    let fh = ifh as f32;
    
    let h = match c {
        &OrthoCamera(_, _, _) => fw.min(fh),
        &PerspectiveCamera(_, _, fov_y) => fh / ((fov_y / 2f32).tan() * 2f32),
    };

    let dim = fw.max(fh).max(h) + 4f32;
    let rays = get_rays(c);
    println!("figure ({});", ix);
    println!("x = linspace ({}, {}, {});", -fw / 2f32, fw / 2f32, fw + 1f32);
    println!("y = linspace ({}, {}, {});", -fh / 2f32, fh / 2f32, fh + 1f32);
    println!("[xx, yy] = meshgrid(x, y);");
    println!("zz = (xx.*0).+{};", h);
    println!("mesh(xx, yy, zz);");
    println!("hold on;");
    println!("grid off;");
    println!("box off;");
    println!("axis ([{}, {}, {}, {}, {}, {}], \"square\");", -dim/2f32, dim/2f32, -dim/2f32, dim/2f32, 0f32, dim);
    println!("daspect ([1, 1, 1]);");
    println!("pbaspect ([1, 1, 1]);");
    println!("title (\"{}\");", title);
    print!("ox = [");
    for r in rays.iter() {
        print!("{}, ", r.origin.x);
    }
    println!("];");
    print!("oy = [");
    for r in rays.iter() {
        print!("{}, ", r.origin.y);
    }
    println!("];");
    print!("oz = [");
    for r in rays.iter() {
        print!("{}, ", r.origin.z);
    }
    println!("];");
    print!("dx = [");
    for r in rays.iter() {
        print!("{}, ", r.direction.x * h * 1.2f32 / r.direction.z);
    }
    println!("];");
    print!("dy = [");
    for r in rays.iter() {
        print!("{}, ", r.direction.y * h * 1.2f32 / r.direction.z);
    }
    println!("];");
    print!("dz = [");
    for r in rays.iter() {
        print!("{}, ", r.direction.z * h * 1.2f32 / r.direction.z);
    }
    println!("];");
    println!("q = quiver3(ox, oy, oz, dx, dy, dz, 0);");
    println!("set (q, \"maxheadsize\", 0);");
    println!("hold off;");
}


