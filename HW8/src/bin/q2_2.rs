use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::Write;

struct PolarPoint {
    r: f64,
    t: f64,
}

impl PolarPoint {
    fn new(r: f64, t: f64) -> PolarPoint {
        PolarPoint { r, t }
    }
}

fn csv_to_cartesian<W: Write>(mut writer: Writer<W>, pt_list: &[PolarPoint]) -> Result<(), Box<dyn Error>> {
    for polar in pt_list {
        let x = polar.r * polar.t.cos();
        let y = polar.r * polar.t.sin();

        writer.write_record(&[x.to_string(), y.to_string()])?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![PolarPoint::new(1.4142135623730951,0.7853981633974483), PolarPoint::new(2.,0.)];

    let file = File::create("cartesian_result.csv")?;
    let csv_writer = csv::Writer::from_writer(file);

    csv_to_cartesian(csv_writer, &points)?;

    Ok(())
}


