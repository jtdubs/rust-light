use light::filters::filter::Filter;
use light::filters::box_filter::BoxFilter;
use light::filters::triangle::TriangleFilter;
use light::filters::gaussian::GaussianFilter;
use light::filters::mitchell::MitchellFilter;
use light::filters::lanczos_sinc::LanczosSincFilter;

fn main() {
    println!("clf");
    test(1, "Box", &BoxFilter::new(2f32, 2f32));
    test(2, "Triangle", &TriangleFilter::new(2f32, 2f32));
    test(3, "Gaussian", &GaussianFilter::new(2f32, 2f32, 0.5f32));
    test(4, "Mitchell", &MitchellFilter::new(3f32, 3f32, 0.5f32, 0.25f32));
    test(5, "Lanczos Sinc", &LanczosSincFilter::new(3f32, 3f32, std::f32::consts::PI));
}

fn test(ix : i64, title : &str, f : &dyn Filter) {
    println!("figure ({});", ix);
    println!("hold on;");
    println!("pbaspect ([1, 1]);");
    println!("axis (\"off\", \"nolabel\");");
    println!("title (\"{}\");", title);
    println!("x = linspace(-3, 3, 61);");
    println!("y = linspace(-3, 3, 61);");
    println!("[xx, yy] = meshgrid(x, y);");
    println!("z = [");
    for y in -30..=30 {
        for x in -30..=30 {
            print!("{}", f.weight((x as f32)/10f32, (y as f32)/10f32));
            if x != 30i32 { print!(","); }
        }
        println!(";");
    }
    println!("];");
    println!("surf(xx, yy, z);");
    println!("hold off;");
}
