struct Point {
    x: f64,
    y: f64,
}

struct PolarPoint {
    r: f64,
    t: f64,
}


////////////// 1.1 //////////////
fn to_polar(pt_list: &Point) -> PolarPoint {
        let r = (pt_list.x.powi(2) + pt_list.y.powi(2)).sqrt();
        let t = (pt_list.y / pt_list.x).atan();
        PolarPoint {r, t}
}


////////////// 1.2 //////////////
fn to_cartesian(pt_list: &PolarPoint) -> Point {
        let x = pt_list.r * pt_list.t.cos();
        let y = pt_list.r * pt_list.t.sin();
        Point {x, y}
}


////////////// Check //////////////
fn main() {
    let cartesian_points = [Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 0.0 }];
    let polar_points = [PolarPoint { r: 1.4142135623730951, t: 0.7853981633974483 }, PolarPoint { r: 2.0, t: 0.0 }];

    let polar_result = to_polar(&cartesian_points[0]);
    let cartesian_result = to_cartesian(&polar_points[0]);

    println!("Converting Point to PolarPoint:");
    println!("Point: x={}, y={}", cartesian_points[0].x, cartesian_points[0].y);
    println!("PolarPoint: r={}, t={}", polar_result.r, polar_result.t);

    println!();

    println!("Converting PolarPoint to Point:");
    println!("PolarPoint: r={}, t={}", polar_points[0].r, polar_points[0].t);
    println!("Point: x={}, y={}", cartesian_result.x, cartesian_result.y);
}

