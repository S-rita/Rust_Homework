fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x_arg = if args.len() < 2 { "N/A" } else { &args[1] };
    let y_arg = if args.len() < 3 { "N/A" } else { &args[2] };

println!("Player 1: {}",x_arg);
println!("Player 2: {}",y_arg);
}