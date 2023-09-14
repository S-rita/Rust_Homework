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

    bubble_sort_points_asc(&mut points);
    bubble_sort_points_des(&mut points);
}

fn bubble_sort_points_asc(points: &mut Vec<(f64, f64)>) {
    let len = points.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if points[j].0 > points[j + 1].0 {
                let temp = points[j];
                points[j] = points[j + 1];
                points[j + 1] = temp;
            }
        }
    }

    println!("Ascending order: {:?}", points);
}

fn bubble_sort_points_des(points: &mut Vec<(f64, f64)>) {
    let len = points.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if points[j].0 < points[j + 1].0 {
                let temp = points[j];
                points[j] = points[j + 1];
                points[j + 1] = temp;
            }
        }
    }
    println!("Descending order: {:?}", points);
}
