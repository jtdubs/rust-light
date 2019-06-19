extern crate light;

use light::cameras::camera::Camera;
use light::cameras::perspective::PerspectiveCamera;
use light::cameras::orthographic::OrthographicCamera;
use light::film::Film;
use light::filters::filter::Filter;
use light::filters::box_filter::BoxFilter;
use light::geometry::ray::Ray;

fn get_rays<F : Filter>(c : &Camera, f : &Film<F>) -> Vec<Ray> {
    let mut res = Vec::with_capacity((f.width * f.height) as usize);
    for x in 0..f.width {
        for y in 0..f.height {
            let cx = ((x as f32 + 0.5f32) / (f.width as f32)) * 2f32 - 1f32;
            let cy = ((y as f32 + 0.5f32) / (f.height as f32)) * 2f32 - 1f32;
            let r = c.cast(cx, cy);
            res.push(r);
        }
    }
    res
}

fn main() {
    println!("clf;");
    let f = Film::new(16, 12, BoxFilter::new(1f32, 1f32));
    draw_p(1, "Perspective (60)", &f, &PerspectiveCamera::new(std::f32::consts::FRAC_PI_3, f.width as f32 / f.height as f32));
    draw_p(2, "Perspective (90)", &f, &PerspectiveCamera::new(std::f32::consts::FRAC_PI_2, f.width as f32 / f.height as f32));
    draw_o(3, "Orthographic", &f, &OrthographicCamera::new(f.height as f32 / 2.0f32, f.width as f32 / f.height as f32));
}

fn draw_p<F : Filter>(ix : usize, title : &str, f : &Film<F>, c : &PerspectiveCamera) {
    let fh = f.height as f32;
    draw(ix, title, f, c, fh / ((c.fov_y / 2f32).tan() * 2f32))
}

fn draw_o<F : Filter>(ix : usize, title : &str, f : &Film<F>, c : &OrthographicCamera) {
    let fw = f.width as f32;
    let fh = f.height as f32;
    draw(ix, title, f, c, fw.min(fh))
}

fn draw<F : Filter>(ix : usize, title : &str, f : &Film<F>, c : &Camera, h : f32) {
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


