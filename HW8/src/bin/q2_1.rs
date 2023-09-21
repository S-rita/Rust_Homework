use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Writer;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn csv_to_polar<W: Write>(mut writer: Writer<W>, pt_list: &[Point]) -> Result<(), Box<dyn Error>> {
    for point in pt_list {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let t = (point.y / point.x).atan();

        writer.write_record(&[r.to_string(), t.to_string()])?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![Point::new(1.0, 1.0), Point::new(2.0, 0.0)];

    let file = File::create("polar_result.csv")?;
    let csv_writer = csv::Writer::from_writer(file);

    csv_to_polar(csv_writer, &points)?;

    Ok(())
}


