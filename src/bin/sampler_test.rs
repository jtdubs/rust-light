use light::sampler::*;

pub fn main() {
    println!("clf");
    draw(1, 2, 3, 1, None, "Centers",    Box::new(StrataCentersSampler2D::new(100, 100)));
    draw(1, 2, 3, 2, None, "Uniform",    Box::new(UniformSampler2D::new(100)));
    draw(1, 2, 3, 3, None, "Strata",     Box::new(StrataSampler2D::new(100, 100)));
    draw(1, 2, 3, 4, None, "LHC",        Box::new(LHCSampler2D::new(100)));
    draw(1, 2, 3, 5, None, "Halton",     Box::new(HaltonSampler2D::new(100)));
    draw(1, 2, 3, 6, None, "Hammersley", Box::new(HammersleySampler2D::new(100)));
}

fn draw(f : u32, w : u32, h : u32, ix : u32, grid : Option<(i32, i32)>, title : &str, mut samples : Box<impl Sampler2D>) {
    println!("figure ({});", f);
    println!("subplot ({}, {}, {});", w, h, ix);
    match grid {
        None => println!("rectangle ();"),
        Some((x, y)) => {
            println!("w = {};", x);
            println!("h = {};", y);
            println!("for x = 0:w-1");
            println!("  for y = 0:h-1");
            println!("    rectangle (\"position\", [x*(1/w), y*(1/h), 1/w, 1/h]);");
            println!("  endfor");
            println!("endfor");
        }
     }

    println!("hold on;");
    println!("pbaspect ([1, 1]);");
    println!("axis (\"off\", \"nolabel\");");
    println!("title (\"{}\");", title);
    print!("xs = [");
    for &(x, _) in samples.get_samples().iter() {
        print!("{}, ", x);
    }
    println!("];");
    print!("ys = [");
    for &(_, y) in samples.get_samples().iter() {
        print!("{}, ", y);
    }
    println!("];");
    println!("scatter(xs, ys, 1);");
    println!("hold off;");
}
