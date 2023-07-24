fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please enter a score");
        std::process::exit(1);
    }

let x = args[1].parse::<f32>().unwrap();

let result = Rust_hw3_1::grades(x);

println!("The grade is {}", result);
}


