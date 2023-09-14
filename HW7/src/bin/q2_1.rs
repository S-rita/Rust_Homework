fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        panic!("Please provide at least one pair of coordinates (x, y).");
    }

    let mut points: Vec<(f64, f64)> = Vec::new();

    for i in 0..args.len() / 2 {
        let x_str = &args[i * 2];
        let y_str = &args[i * 2 + 1];

        let x = x_str.parse::<f64>();
        let y = y_str.parse::<f64>();

        match (x, y) {
            (Ok(x), Ok(y)) => {
                points.push((x, y));
            }
            _ => {
                eprintln!("Invalid coordinates: {} {}", x_str, y_str);
                std::process::exit(1);
            }
        }
    }

    if points.is_empty() {
        panic!("No valid pairs of coordinates provided.");
    }

    sort_points(&mut points);
}

fn sort_points(points: &mut Vec<(f64, f64)>) {
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("Ascending order: {:?}", points);

    points.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("Descending order: {:?}", points);
}
