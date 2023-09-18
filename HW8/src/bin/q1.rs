////////////// 1.1 //////////////
fn to_polar(pt_list: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut PolarPoint = Vec::new();

    for (x, y) in pt_list {
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let t = (y / x).atan();
        
        PolarPoint.push((r, t));
    }
    PolarPoint
}


////////////// 1.2 //////////////
fn to_cartesian(pt_list: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut Point = Vec::new();

    for (r, t) in pt_list {
        let x = r * t.cos();
        let y = r * t.sin();
        Point.push((x, y));
    }
    Point
}

////////////// Check //////////////
fn main(){
    let cartesian_points = [(1.0, 1.0), (2.0, 0.0)];
    let polar_points = [(1.4142135623730951, 0.7853981633974483), (2.0, 0.0)];

    let polar_result = to_polar(&cartesian_points);
    let cartesian_result = to_cartesian(&polar_points);

    println!("{:?}", polar_result);
    println!("{:?}", cartesian_result);

}

