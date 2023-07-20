fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x_arg = if args.len() < 2 { "" } else { &args[1]};

    let x: f32 = x_arg.parse().unwrap_or(0.0);

println!("{} degree Fahrenheit converts to {} degree Celcius", x, calculate_celsius(x));
}

fn calculate_celsius(f: f32) -> f32 {
    return (5.0/9.0)*(f - 32.0);
}
