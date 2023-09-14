fn sort_numbers(numbers: &mut Vec<f64>) {
    if numbers.len() < 2 {
        panic!("Please provide at least two numbers");
    }

    let mut asc = numbers.clone();
    bubble_sort_asc(&mut asc);
    println!("Ascending order: {:?}", asc);

    let mut des = numbers.clone();
    bubble_sor_des(&mut des);
    println!("Descending order: {:?}", des);
}

fn bubble_sort_asc(numbers: &mut Vec<f64>) {
    let len = numbers.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sor_des(numbers: &mut Vec<f64>) {
    let len = numbers.len();
    for i in 0..len {
        for j in 0..len - 1 - i{
            if numbers[j] < numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut numbers: Vec<f64> = args
        .iter()
        .map(|arg| {
            arg.parse::<f64>()
                .expect("Invalid number in command line arguments")
        })
        .collect();

    sort_numbers(&mut numbers);
}