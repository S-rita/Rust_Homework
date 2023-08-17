fn main(){
}


//----------------------2.1

fn split_grades(x: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut above_d = Vec::new();
    let mut d_f = Vec::new();

    for grade in x {
        match grade {
            "A+" | "A" | "B" | "C" => above_d.push(grade),
            "D" | "F" => d_f.push(grade),
            _ => {}
        }
    }
    (above_d, d_f)
}

#[test]
fn test_split_grades() {
    assert_eq!(
        split_grades((&["A+", "B", "C", "D", "F"]).to_vec()),
        (
            vec!["A+", "B", "C"],
            vec!["D", "F"]
        )
    );
    assert_eq!(
        split_grades((&["F", "B", "A++", "D", "A+", "F-", "a"]).to_vec()),
        (
            vec!["B", "A+"],
            vec!["F", "D"]
        )
    );
}



//----------------------2.2

fn split_scores(x: &[i32]) -> (Vec<(&'static str, i32)>, Vec<(&'static str, i32)>) {
    let mut above_d = Vec::new();
    let mut d_f = Vec::new();

    for &score in x.iter() {
        let grade = match score {
            95..=100 => "A+",
            81..=94 => "A",
            71..=80 => "B",
            61..=70 => "C",
            50..=60 => "D",
            0..=49 => "F",
            _ => "Invalid score",
        };

        if grade == "D" || grade == "F" {
            d_f.push((grade, score));
        } else if grade == "A+" || grade == "A" || grade == "B" || grade == "C" {
            above_d.push((grade, score));
        } else {
            _ = Vec::<String>::new();
        }
    }

    (above_d, d_f)
}

#[test]
fn test_split_scores() {
    assert_eq!(
        split_scores(&[75, 42, 98, 54, 63]),
        (
            vec![("B", 75), ("A+", 98), ("C", 63)],
            vec![("F", 42), ("D", 54)]
        )
    );
    assert_eq!(
        split_scores(&[-100, 0, 20, 100, 200]),
        (
            vec![("A+", 100)],
            vec![("F", 0), ("F", 20)]
        )
    );
}
