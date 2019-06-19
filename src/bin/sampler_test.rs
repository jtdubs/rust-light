use light::sampler::Sampler;

pub fn main() {
    let mut s = Sampler::new();

    println!("clf");
    draw(1, 2, 3, 1, None, "Centers",    &s.strata_centers_2d(10, 10));
    draw(1, 2, 3, 2, None, "Uniform",    &s.uniform_2d(100));
    draw(1, 2, 3, 3, None, "Strata",     &s.strata_2d(10, 10));
    draw(1, 2, 3, 4, None, "LHC",        &s.lhc_2d(100));
    draw(1, 2, 3, 5, None, "Halton",     &s.halton_2d(100));
    draw(1, 2, 3, 6, None, "Hammersley", &s.hammersley_2d(100));
}

fn draw(f : u32, w : u32, h : u32, ix : u32, grid : Option<(i32, i32)>, title : &str, samples : &Vec<(f32, f32)>) {
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
    for &(x, _) in samples.iter() {
        print!("{}, ", x);
    }
    println!("];");
    print!("ys = [");
    for &(_, y) in samples.iter() {
        print!("{}, ", y);
    }
    println!("];");
    println!("scatter(xs, ys, 1);");
    println!("hold off;");
}
