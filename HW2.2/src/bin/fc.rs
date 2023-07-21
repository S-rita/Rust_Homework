fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No Fahrenheit specified");
    }
    let x_arg: &String = &args[1];
    let x: f32 = x_arg.parse().unwrap_or(0.0);

println!("{} degree Fahrenheit converts to {} degree Celcius", x, calculate_celsius(x));
}

fn calculate_celsius(f: f32) -> f32 {
    return (5.0/9.0)*(f - 32.0);
}
