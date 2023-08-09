//2.1

fn make_grades(scores: &[i64]) -> Vec<&'static str> {
    let mut grades = Vec::new();

    for &i in scores {
        let result = match i {
            95..=100 => "Excellent with A+",
            81..=94 => "A",
            71..=80 => "B",
            61..=70 => "C",
            50..=60 => "D",
            0..=49 => "Failed with F",
            _ => "Invalid score",
        };
        grades.push(result);
    }
    grades
}

#[test]
fn test_make_grades() {
    assert_eq!(make_grades(&[101]), ["Invalid score"]);
    assert_eq!(make_grades(&[97]), ["Excellent with A+"]);
    assert_eq!(make_grades(&[71]), ["B"]);
    assert_eq!(make_grades(&[69]), ["C"]);
    assert_eq!(make_grades(&[46]), ["Failed with F"]);
    assert_eq!(make_grades(&[-10]), ["Invalid score"]);
}


//2.2

fn make_grades_recursive(scores: &[i64]) -> Vec<&'static str> {
    if scores.is_empty() {
        Vec::new()
    } else {
        let x = scores[0];
        let result =  match x {
            95..=100 => "Excellent with A+",
            81..=94 => "A",
            71..=80 => "B",
            61..=70 => "C",
            50..=60 => "D",
            0..=49 => "Failed with F",
            _ => "Invalid score",
        };
        let mut grades = make_grades_recursive(&scores[1..]);
        grades.push(result);
        grades
    }
}

#[test]
fn test_make_grades_recursive() {
    assert_eq!(make_grades_recursive(&[101]), ["Invalid score"]);
    assert_eq!(make_grades_recursive(&[97]), ["Excellent with A+"]);
    assert_eq!(make_grades_recursive(&[71]), ["B"]);
    assert_eq!(make_grades_recursive(&[69]), ["C"]);
    assert_eq!(make_grades_recursive(&[46]), ["Failed with F"]);
    assert_eq!(make_grades_recursive(&[-10]), ["Invalid score"]);
}

fn main() {
}