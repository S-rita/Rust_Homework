//1.1//

fn fahr_to_cel_v(v: &[f64]) -> Vec<f64> {
    let mut c = Vec::new();

    for i in v {
        let result = (5.0 / 9.0) * (i - 32.0);
        c.push(result);
    }
    c
}

#[test]
fn test_fahr_to_cel_v() {
    assert_eq!(fahr_to_cel_v(&[]), []);
    assert_eq!(fahr_to_cel_v(&[32.0, 68.0, 140.0]), [0.0, 20.0, 60.0]);
    assert_eq!(fahr_to_cel_v(&[-4.0, -40.0, -76.0]), [-20.0, -40.0, -60.0]);
}



//1.2//

fn fahr_to_cel_v_recursive(v: &[f64]) -> Vec<f64> {
    if v.is_empty() {
        Vec::new()
    } else {
        let calc = (5.0 / 9.0) * (v[0] - 32.0);
        let mut c = fahr_to_cel_v_recursive(&v[1..]);
        c.insert(0, calc);
        c
    }
}

#[test]
fn test_fahr_to_cel_v_recursive() {
    assert_eq!(fahr_to_cel_v_recursive(&[]), []);
    assert_eq!(fahr_to_cel_v_recursive(&[32.0, 68.0, 140.0]), [0.0, 20.0, 60.0]);
    assert_eq!(fahr_to_cel_v_recursive(&[-4.0, -40.0, -76.0]), [-20.0, -40.0, -60.0]);
}

fn main(){
    
}