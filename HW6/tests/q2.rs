fn main() {

}

//////////////////////// 2.1 ////////////////////////

fn pack_number_tuples3(x: &[i32], y: &[i32], z: &[i32] ) -> Vec<(i32, i32, i32)> {

    let mut result = Vec::new();
    let lenx = x.len();
    let leny = y.len();
    let lenz = z.len();
    let max = lenx.max(leny.max(lenz));

    for i in 0..max {
        let vx = if i < lenx { x[i] } else { 0 };
        let vy = if i < leny { y[i] } else { 0 };
        let vz = if i < lenz { z[i] } else { 0 };
        result.push((vx, vy, vz))
    }
    result
}

#[test]
fn test_pack_number_tuples3() {
    assert_eq!(pack_number_tuples3(&[], &[], &[]), vec![]);
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]), vec![(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[2], &[3, 4, 5], &[]), vec![(2, 3, 0), (0, 4, 0), (0, 5, 0)]);
}


//////////////////////// 2.2 ////////////////////////

fn pack_number_tuples_s3(x: &[i32], y: &[i32], z: &[i32] ) -> Vec<(i32, i32, i32)> {

    let mut result = Vec::new();
    let lenx = x.len();
    let leny = y.len();
    let lenz = z.len();
    let min = lenx.min(leny.min(lenz));

    for i in 0..min {
        let vx = if i < lenx { x[i] } else { 0 };
        let vy = if i < leny { y[i] } else { 0 };
        let vz = if i < lenz { z[i] } else { 0 };
        result.push((vx, vy, vz))
    }
    result
}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), vec![]);
    assert_eq!(pack_number_tuples_s3(&[2], &[3, 4, 5], &[]), vec![]);
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]), vec![(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&[-1, 2, 3], &[4], &[5]), vec![(-1, 4, 5)]);
}