use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    eprintln!("Please provide a number as an argument");
    std::process::exit(1);
    }

    let n = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a positive number as an argument");
            std::process::exit(1);
        }
    };

    for i in 1..=n {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }

    for i in (1..n).rev() {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
    
}
