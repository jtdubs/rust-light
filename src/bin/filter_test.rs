extern crate light;

use light::filter::Filter;
use std::iter::range_inclusive;

fn main() {
    println!("clf");
    test(1, "Box", Filter::new_box(2f64, 2f64));
    test(2, "Triangle", Filter::new_triangle(2f64, 2f64));
    test(3, "Gaussian", Filter::new_gaussian(2f64, 2f64, 0.5f64));
    test(4, "Mitchell", Filter::new_mitchell(3f64, 3f64, 0.5f64, 0.25f64));
    test(5, "Lanczos Sinc", Filter::new_lanczos_sinc(3f64, 3f64, Float::pi()));
}

fn test(ix : i64, title : &str, f : Filter) {
    println!("figure ({});", ix);
    println!("hold on;");
    println!("pbaspect ([1, 1]);");
    println!("axis (\"off\", \"nolabel\");");
    println!("title (\"{}\");", title);
    println!("x = linspace(-3, 3, 61);");
    println!("y = linspace(-3, 3, 61);");
    println!("[xx, yy] = meshgrid(x, y);");
    println!("z = [");
    for y in range_inclusive(-30i, 30i) {
        for x in range_inclusive(-30i, 30i) {
            print!("{}", f.weight((x as f64)/10f64, (y as f64)/10f64));
            if x != 30i { print!(","); }
        }
        println!(";");
    }
    println!("];");
    println!("surf(xx, yy, z);");
    println!("hold off;");
}
