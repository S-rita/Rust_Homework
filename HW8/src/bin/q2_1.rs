use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn csv_to_polar<W: Write>(mut writer: Writer<W>, pt_list: &[(f64, f64)]) -> Result<(), Box<dyn Error>> {
    for (x, y) in pt_list {
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let t = (y / x).atan();

        writer.write_record(&[r.to_string(), t.to_string()])?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![(1.0, 1.0), (2.0, 0.)];

    let file = File::create("polar_result.csv")?;
    let csv_writer = csv::Writer::from_writer(file);

    csv_to_polar(csv_writer, &points)?;

    Ok(())
}

