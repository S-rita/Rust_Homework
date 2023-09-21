struct Point {
    x: f64,
    y: f64,
}

struct PolarPoint {
    r: f64,
    t: f64,
}

////////////// 1.1 //////////////
fn to_polar(point: &Point) -> PolarPoint {
    let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
    let t = point.y.atan();
    PolarPoint { r, t }
}


////////////// 1.2 //////////////
fn to_cartesian(polar_point: &PolarPoint) -> Point {
    let x = polar_point.r * polar_point.t.cos();
    let y = polar_point.r * polar_point.t.sin();
    Point { x, y }
}


////////////// Check //////////////
fn main() {
    let cartesian_points = [Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 0.0 }];
    let polar_points = [PolarPoint { r: 1.4142135623730951, t: 0.7853981633974483 }, PolarPoint { r: 2.0, t: 0.0 }];

    let polar_result = to_polar(&cartesian_points[0]);
    let cartesian_result = to_cartesian(&polar_points[0]);

    println!("{:?}", polar_result);
    println!("{:?}", cartesian_result);
}