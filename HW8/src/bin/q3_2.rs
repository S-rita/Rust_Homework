use std::fs::File;
use std::io::{Result, Write};

struct PolarPoint {
    r: f64,
    t: f64,
}

impl PolarPoint {
    fn new(r: f64, t: f64) -> PolarPoint {
        PolarPoint { r, t }
    }
}

fn main() -> Result<()> {
    let points = vec![
        PolarPoint::new(2.23606797749979, 1.1071487177940906),
        PolarPoint::new(1.4142135623730951, 0.7853981633974483),
        PolarPoint::new(2.0, 0.0),
        PolarPoint::new(2.8284271247461903, 0.7853981633974483),
    ];

    let html_file = File::create("cartesian_result.html")?;

    let mut html_writer = std::io::BufWriter::new(html_file);
    cartesain_to_html(&mut html_writer, &points)?;

    Ok(())
}

fn cartesain_to_html<W: Write>(mut writer: W, pt_list: &[PolarPoint]) -> Result<()> {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    html.push_str("<table width=50%>\n");

    html.push_str("<tr>\n");
    html.push_str("  <th>(x, y)</th>\n");
    html.push_str("  <th>(r, t)</th>\n");
    html.push_str("</tr>\n");

    for polar in pt_list {
        let x = polar.r * polar.t.cos();
        let y = polar.r * polar.t.sin();

        html.push_str("<tr>\n");
        html.push_str(&format!("  <td align=center>({:?}, {:?})</td>\n", polar.r, polar.t));
        html.push_str(&format!("  <td align=center>({:.1}, {:.1})</td>\n", x, y));
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    writer.write_all(html.as_bytes())?;

    Ok(())
}

