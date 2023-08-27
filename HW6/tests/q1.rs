fn main() {
}

//////////////////////// 1.1 ////////////////////////

fn min_max_avg(x: &[i32]) -> Vec<(i32, i32, i32)> {
    if x.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut min = x[0];
    let mut max = x[0];
    let mut sum = 0;

    for &i in x.iter() {
        if i < min {
            min = i;
        }
        if i > max {
            max = i;
        }
        sum += i;
    }

    let avg = sum / x.len() as i32;
    result.push((min, max, avg));
    result
}

#[test]
fn test_min_max_avg() {
    assert_eq!(min_max_avg(&[]), vec![]);
    assert_eq!(min_max_avg(&[1, 2, 3, 4, 5]), vec![(1, 5, 3)]);
    assert_eq!(min_max_avg(&[-100, 0, -200, 100, 200]), vec![(-200, 200, 0)]);
}


//////////////////////// 1.2 ////////////////////////

fn cal_partial_sums(x: &[i32]) -> Vec<i32> {
    if x.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut sum = 0;

    for &i in x.iter() {
        sum += i;
        result.push(sum);
    }
    result
}

#[test]
fn test_partial_sums() {
    assert_eq!(cal_partial_sums(&[]), vec![]);
    assert_eq!(cal_partial_sums(&[2, 11, 3, 4, 7]), vec![2, 13, 16, 20 ,27]);
    assert_eq!(cal_partial_sums(&[-11, 0, 11, 22, 33]), vec![-11, -11, 0, 22, 55]);
}