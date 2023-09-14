fn main() {
    let table = generate_html_table();
    println!("{}", table);
}

fn generate_html_table() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        panic!("Please enter three numbers: start, end, step");
        }

    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    html.push_str("<table width =40%>\n");

    html.push_str("<tr>\n");
    html.push_str("  <th>x</th>\n");
    html.push_str("  <th>x²</th>\n");
    html.push_str("  <th>x³</th>\n");
    html.push_str("</tr>\n");

    let start: i32 = args[0].parse().unwrap();
    let end: i32 = args[1].parse().unwrap();
    let step: i32 = args[2].parse().unwrap();

    let mut current_value = start;

    while current_value <= end {

        html.push_str("<tr>\n");

        let formatted_x = format!("  <td align=center>{}</td>\n", current_value);
        html.push_str(&formatted_x);

        let formatted_squared = format!("  <td align=center>{}</td>\n", current_value.pow(2));
        html.push_str(&formatted_squared);

        let formatted_cubed = format!("  <td align=center>{}</td>\n", current_value.pow(3));
        html.push_str(&formatted_cubed);

        current_value += step;

        html.push_str("</tr>\n");

    }

    while current_value >= end {

        html.push_str("<tr>\n");

        let formatted_x = format!("  <td align=center>{}</td>\n", current_value);
        html.push_str(&formatted_x);

        let formatted_squared = format!("  <td align=center>{}</td>\n", current_value.pow(2));
        html.push_str(&formatted_squared);

        let formatted_cubed = format!("  <td align=center>{}</td>\n", current_value.pow(3));
        html.push_str(&formatted_cubed);

        current_value -= step;

        html.push_str("</tr>\n");

    }
    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    html
}
