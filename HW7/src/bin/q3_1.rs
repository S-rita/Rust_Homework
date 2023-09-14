fn main() {
    let table = generate_html_table();
    println!("{}", table);
}

fn generate_html_table() -> String {
    // Collect command-line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        panic!("Please enter three numbers: start, end, step");
    }

    let mut html = String::new();

    // Start the HTML document
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    // Start the table
    html.push_str("<table width =40%>\n");

    // Table headers
    html.push_str("<tr>\n");
    html.push_str("  <th>Fahr</th>\n");
    html.push_str("  <th>Celsius</th>\n");
    html.push_str("</tr>\n");

    // Parse the input arguments
    let start: i32 = args[0].parse().unwrap();
    let end: i32 = args[1].parse().unwrap();
    let step: i32 = args[2].parse().unwrap();

    let mut current_value = start;

    while current_value <= end {
        let celsius = (current_value as f64 - 32.0) * 5.0 / 9.0;

        html.push_str("<tr>\n");

        let formatted_fahrenheit = format!("  <td align=center>{}</td>\n", current_value);
        html.push_str(&formatted_fahrenheit);

        let formatted_celsius = format!("  <td align=center>{:.1}</td>\n", celsius);
        html.push_str(&formatted_celsius);

        html.push_str("</tr>\n");

        current_value -= step;
    }
    
    while current_value >= end {
        let celsius = (current_value as f64 - 32.0) * 5.0 / 9.0;

        html.push_str("<tr>\n");

        let formatted_fahrenheit = format!("  <td align=center>{}</td>\n", current_value);
        html.push_str(&formatted_fahrenheit);

        let formatted_celsius = format!("  <td align=center>{:.1}</td>\n", celsius);
        html.push_str(&formatted_celsius);

        html.push_str("</tr>\n");

        current_value -= step;
    }

    // Close the table and the HTML document
    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    html
}
