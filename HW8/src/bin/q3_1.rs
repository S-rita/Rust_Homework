use std::fs::File;
use std::io::{Result, Write};

fn main() -> Result<()> {
    let points = vec![
        (-1., -2.),
        (1.0, 1.0),
        (2.0, 0.0),
        (2.0, 2.0)
    ];

    let html_file = File::create("polar_result.html")?;

    let mut html_writer = std::io::BufWriter::new(html_file);
    polar_to_html(&mut html_writer, &points)?;

    Ok(())
}

fn polar_to_html<W: Write>(mut writer: W, pt_list: &[(f64, f64)]) -> Result<()> {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    html.push_str("<table width=30%>\n");

    html.push_str("<tr>\n");
    html.push_str("  <th>(x, y)</th>\n");
    html.push_str("  <th>(r, t)</th>\n");
    html.push_str("</tr>\n");

    for (x, y) in pt_list {
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let t = (y / x).atan();

        html.push_str("<tr>\n");
        html.push_str(&format!("  <td align=center>({:?}, {:?})</td>\n", x, y));
        html.push_str(&format!("  <td align=center>({:?}, {:?})</td>\n", r, t));
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    writer.write_all(html.as_bytes())?;

    Ok(())
}





