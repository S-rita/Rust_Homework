fn main() {

}

//////////////////////// 3.1 ////////////////////////

fn unpack_number_tuples(x: &[(i32, i32)] ) -> (Vec<i32>, Vec<i32>) {

    let mut result1 = Vec::new();
    let mut result2 = Vec::new();

    for &(a, b) in x {
        result1.push(a);
        result2.push(b);
    }

    (result1, result2)
}

#[test]
fn test_unpack_number_tuples() {
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(
        unpack_number_tuples(&[(1, 4), (3, 2), (2, 1)]),
        (vec![1, 3, 2], vec![4, 2, 1])
    );
    assert_eq!(
        unpack_number_tuples(&[(-1, 0), (0, 2), (3, 0)]),
        (vec![-1, 0, 3], vec![0, 2, 0])
    );
}


//////////////////////// 3.2 ////////////////////////

fn unpack_number_tuples3(x: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {

    let mut result1 = Vec::new();
    let mut result2 = Vec::new();
    let mut result3 = Vec::new();

    for &(a, b, c) in x {
        result1.push(a);
        result2.push(b);
        result3.push(c);
    }
    (result1, result2, result3)
}

#[test]
fn test_unpack_number_tuples3() {
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(
        unpack_number_tuples3(&[(1, 4, 5), (-2, 2, 1)]),
        (vec![1, -2], vec![4, 2], vec![5, 1])
    );
}