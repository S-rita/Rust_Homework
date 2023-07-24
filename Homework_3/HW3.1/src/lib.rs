pub fn grades(x: f32) -> String {
    match x {
            95.0..=100.0 => String::from("Excellent with A+"),
            81.0..=94.0 => String::from("A"),
            71.0..=80.0 => String::from("B"),
            61.0..=70.0 => String::from("C"),
            50.0..=60.0 => String::from("D"),
            0.0..=49.0 => String::from("Failed with F"),
            _ => String::from("Invalid score"),
        }
}

