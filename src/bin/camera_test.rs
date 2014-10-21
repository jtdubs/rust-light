extern crate light;

use std::num::FloatMath;
use light::camera::{Camera,OrthoCamera,PerspectiveCamera};
use light::film::Film;
use light::filter::BoxFilter;
use light::geometry::ray::Ray;

fn get_rays<'a>(c : &'a Camera<'a>) -> Vec<Ray> {
    let f = c.get_film();
    let mut res = Vec::with_capacity((f.width * f.height) as uint);
    for x in range(0, f.width) {
        for y in range(0, f.height) {
            let r = c.cast((x as f64) + 0.5f64, (y as f64) + 0.5f64);
            res.push(r);
        }
    }
    res
}

fn main() {
    let bf = &BoxFilter::new(1f64, 1f64);
    let f = &Film::new(16, 12, bf);

    println!("clf;");
    draw_perspective(1, "Perspective (60)", &PerspectiveCamera::new(f, Float::frac_pi_3()));
    draw_perspective(2, "Perspective (90)", &PerspectiveCamera::new(f, Float::frac_pi_2()));
    draw_ortho(3, "Orthographic", &OrthoCamera::new(f, 1f64));
}

fn draw_perspective<'a>(ix : int, title : &str, c : &'a PerspectiveCamera<'a>) {
    let f = c.get_film();
    draw_plot(ix, title, c, (f.height as f64) / ((c.get_fov_y() / 2f64).tan() * 2f64));
}

fn draw_ortho<'a>(ix : int, title : &str, c : &'a OrthoCamera<'a>) {
    let f = c.get_film();
    draw_plot(ix, title, c, (f.width as f64).min(f.height as f64));
}

fn draw_plot<'a>(ix : int, title : &str, c : &'a Camera<'a>, h : f64) {
    let f = c.get_film();
    let fw = f.width as f64;
    let fh = f.height as f64;
    let dim = fw.max(fh).max(h) + 4f64;
    let rays = get_rays(c);
    println!("figure ({});", ix);
    println!("x = linspace ({}, {}, {});", -fw / 2f64, fw / 2f64, fw + 1f64);
    println!("y = linspace ({}, {}, {});", -fh / 2f64, fh / 2f64, fh + 1f64);
    println!("[xx, yy] = meshgrid(x, y);");
    println!("zz = (xx.*0).+{};", h);
    println!("mesh(xx, yy, zz);");
    println!("hold on;");
    println!("grid off;");
    println!("box off;");
    println!("axis ([{}, {}, {}, {}, {}, {}], \"square\");", -dim/2f64, dim/2f64, -dim/2f64, dim/2f64, 0f64, dim);
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
        print!("{}, ", r.direction.x * h * 1.2f64 / r.direction.z);
    }
    println!("];");
    print!("dy = [");
    for r in rays.iter() {
        print!("{}, ", r.direction.y * h * 1.2f64 / r.direction.z);
    }
    println!("];");
    print!("dz = [");
    for r in rays.iter() {
        print!("{}, ", r.direction.z * h * 1.2f64 / r.direction.z);
    }
    println!("];");
    println!("q = quiver3(ox, oy, oz, dx, dy, dz, 0);");
    println!("set (q, \"maxheadsize\", 0);");
    println!("hold off;");
}


