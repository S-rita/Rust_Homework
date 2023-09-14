fn sort_numbers(numbers: &mut Vec<f64>) {
    if numbers.len() < 2 {
        panic!("Please provide at least two numbers");
    }
    let mut asc = numbers.clone();
    asc.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Ascending order: {:?}", asc);

    let mut des = numbers.clone();
    des.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Descending order: {:?}", des);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut numbers: Vec<f64> = args
        .iter()
        .map(|arg|
            arg.parse::<f64>()
                .expect("Invalid number in command line arguments")
        )
        .collect();

    sort_numbers(&mut numbers);
}