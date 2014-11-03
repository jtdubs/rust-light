extern crate light;

use std::num::FloatMath;
use light::camera::Camera;
use light::perspective_camera::PerspectiveCamera;
use light::orthographic_camera::OrthographicCamera;
use light::film::Film;
use light::filter::Filter;
use light::ray::Ray;

fn get_rays(c : &Camera, f : &Film) -> Vec<Ray> {
    let mut res = Vec::with_capacity((f.width * f.height) as uint);
    for x in range(0, f.width) {
        for y in range(0, f.height) {
            let r = c.cast(f, (x as f32) + 0.5f32, (y as f32) + 0.5f32);
            res.push(r);
        }
    }
    res
}

fn main() {
    println!("clf;");
    let f = Film::new(16, 12, Filter::new_box(1f32, 1f32));
    draw_p(1, "Perspective (60)", &f, &PerspectiveCamera::new(Float::frac_pi_3(), (f.width as f32 / f.height as f32)));
    draw_p(2, "Perspective (90)", &f, &PerspectiveCamera::new(Float::frac_pi_2(), (f.width as f32 / f.height as f32)));
    draw_o(3, "Orthographic", &f, &OrthographicCamera::new(1f32));
}

fn draw_p(ix : int, title : &str, f : &Film, c : &PerspectiveCamera) {
    let fh = f.height as f32;
    draw(ix, title, f, c, fh / ((c.fov_y / 2f32).tan() * 2f32))
}

fn draw_o(ix : int, title : &str, f : &Film, c : &OrthographicCamera) {
    let fw = f.width as f32;
    let fh = f.height as f32;
    draw(ix, title, f, c, fw.min(fh))
}

fn draw(ix : int, title : &str, f : &Film, c : &Camera, h : f32) {
    let fw = f.width as f32;
    let fh = f.height as f32;
    
    let dim = fw.max(fh).max(h) + 4f32;
    let rays = get_rays(c, f);
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


