pub fn grades(x: i32) -> String {
    match x {
            95..=100 => String::from("Excellent with A+"),
            81..=94 => String::from("A"),
            71..=80 => String::from("B"),
            61..=70 => String::from("C"),
            50..=60 => String::from("D"),
            0..=49 => String::from("Failed with F"),
            _ => String::from("Invalid score"),
        }
}

