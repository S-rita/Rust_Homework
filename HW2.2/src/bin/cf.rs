fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No Celcius specified");
    }
    let y_arg: &String = &args[1];
    let y: f32 = y_arg.parse().unwrap_or(0.0);

println!("{} degree Celcius converts to {} degree Fahrenheit", y, calculate_fahrenheit(y));
}

fn calculate_fahrenheit(c: f32) -> f32 {
    return (9.0/5.0)*c + 32.0;
}
