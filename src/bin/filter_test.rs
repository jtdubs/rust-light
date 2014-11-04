extern crate light;

use light::filters::filter::Filter;
use light::filters::box_filter::BoxFilter;
use light::filters::triangle::TriangleFilter;
use light::filters::gaussian::GaussianFilter;
use light::filters::mitchell::MitchellFilter;
use light::filters::lanczos_sinc::LanczosSincFilter;
use std::iter::range_inclusive;

fn main() {
    println!("clf");
    test(1, "Box", box BoxFilter::new(2f32, 2f32));
    test(2, "Triangle", box TriangleFilter::new(2f32, 2f32));
    test(3, "Gaussian", box GaussianFilter::new(2f32, 2f32, 0.5f32));
    test(4, "Mitchell", box MitchellFilter::new(3f32, 3f32, 0.5f32, 0.25f32));
    test(5, "Lanczos Sinc", box LanczosSincFilter::new(3f32, 3f32, Float::pi()));
}

fn test(ix : i64, title : &str, f : Box<Filter>) {
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
            print!("{}", f.weight((x as f32)/10f32, (y as f32)/10f32));
            if x != 30i { print!(","); }
        }
        println!(";");
    }
    println!("];");
    println!("surf(xx, yy, z);");
    println!("hold off;");
}
