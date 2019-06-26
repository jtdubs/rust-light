use light::sampler::*;

pub fn main() {
    println!("clf");

    draw(1, 2, 3, 1, false, "Centers",    StrataCentersSampler2D::new(10, 10).get_samples());
    draw(1, 2, 3, 2, false, "Uniform",    UniformSampler2D::new(100).get_samples());
    draw(1, 2, 3, 3, false, "Strata",     StrataSampler2D::new(10, 10).get_samples());
    draw(1, 2, 3, 4, false, "LHC",        LHCSampler2D::new(100).get_samples());
    draw(1, 2, 3, 5, false, "Halton",     HaltonSampler2D::new(100).get_samples());
    draw(1, 2, 3, 6, false, "Hammersley", HammersleySampler2D::new(100).get_samples());

    draw(2, 2, 3, 1, true, "Centers (U)",    StrataCentersSampler2D::new(10, 10).get_samples().into_iter().map(to_disc_uniform).collect());
    draw(2, 2, 3, 2, true, "Uniform (U)",    UniformSampler2D::new(100).get_samples().into_iter().map(to_disc_uniform).collect());
    draw(2, 2, 3, 3, true, "Strata (U)",     StrataSampler2D::new(10, 10).get_samples().into_iter().map(to_disc_uniform).collect());
    draw(2, 2, 3, 4, true, "LHC (U)",        LHCSampler2D::new(100).get_samples().into_iter().map(to_disc_uniform).collect());
    draw(2, 2, 3, 5, true, "Halton (U)",     HaltonSampler2D::new(100).get_samples().into_iter().map(to_disc_uniform).collect());
    draw(2, 2, 3, 6, true, "Hammersley (U)", HammersleySampler2D::new(100).get_samples().into_iter().map(to_disc_uniform).collect());

    draw(3, 2, 3, 1, true, "Centers (C)",    StrataCentersSampler2D::new(10, 10).get_samples().into_iter().map(to_disc_concentric).collect());
    draw(3, 2, 3, 2, true, "Uniform (C)",    UniformSampler2D::new(100).get_samples().into_iter().map(to_disc_concentric).collect());
    draw(3, 2, 3, 3, true, "Strata (C)",     StrataSampler2D::new(10, 10).get_samples().into_iter().map(to_disc_concentric).collect());
    draw(3, 2, 3, 4, true, "LHC (C)",        LHCSampler2D::new(100).get_samples().into_iter().map(to_disc_concentric).collect());
    draw(3, 2, 3, 5, true, "Halton (C)",     HaltonSampler2D::new(100).get_samples().into_iter().map(to_disc_concentric).collect());
    draw(3, 2, 3, 6, true, "Hammersley (C)", HammersleySampler2D::new(100).get_samples().into_iter().map(to_disc_concentric).collect());
}

fn draw(f : u32, w : u32, h : u32, ix : u32, disc : bool, title : &str, samples : Vec<(f32, f32)>) {
    println!("figure ({});", f);
    println!("subplot ({}, {}, {});", w, h, ix);

    if disc {
        println!("t = linspace (0, 2*pi, 100);");
        println!("circsx = cos(t);");
        println!("circsy = sin(t);");
        println!("plot (circsx, circsy);");
    } else {
        println!("rectangle ();");
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
