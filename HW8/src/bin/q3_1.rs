use std::fs::File;
use std::io::{Result, Write};

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn main() -> Result<()> {
    let points = vec![
        Point::new(-1.0, -2.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 0.0),
        Point::new(2.0, 2.0)
    ];

    let html_file = File::create("polar_result.html")?;

    let mut html_writer = std::io::BufWriter::new(html_file);
    polar_to_html(&mut html_writer, &points)?;

    Ok(())
}

fn polar_to_html<W: Write>(mut writer: W, pt_list: &[Point]) -> Result<()> {
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

    for point in pt_list {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let t = (point.y / point.x).atan();

        html.push_str("<tr>\n");
        html.push_str(&format!("  <td align=center>({:?}, {:?})</td>\n", point.x, point.y));
        html.push_str(&format!("  <td align=center>({:?}, {:?})</td>\n", r, t));
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    writer.write_all(html.as_bytes())?;

    Ok(())
}






