fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { "" } else { &args[1]};

    let x_arg: &String = &args[1];
    let x: f32 = x_arg.parse().unwrap();

println!("The area of a circle: {}", calculate_area(x));
}

fn calculate_area(r: f32) -> f32 {
    const PI: f32 = 3.1416;
    return PI * r * r;
}
