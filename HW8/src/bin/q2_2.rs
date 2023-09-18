use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn csv_to_cartesian<W: Write>(mut writer: Writer<W>, pt_list: &[(f64, f64)]) -> Result<(), Box<dyn Error>> {
    for (r, t) in pt_list {
        let x = r * t.cos();
        let y = r * t.sin();

        writer.write_record(&[x.to_string(), y.to_string()])?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![(1.4142135623730951,0.7853981633974483), (2.,0.)];

    let file = File::create("cartesian_result.csv")?;
    let csv_writer = csv::Writer::from_writer(file);

    csv_to_cartesian(csv_writer, &points)?;

    Ok(())
}

