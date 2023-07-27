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

    for i in 1..=(n * 2 - 1) {
        let n_space = if i <= n {
            n - i
        } else {
            i - n
        };

        for _ in 0..n_space {
            print!(" ");
        }

        for _ in 0..(n - n_space) {
            print!("*");
        }
        println!();

    }
}