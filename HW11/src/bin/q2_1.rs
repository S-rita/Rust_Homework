use std::fs;
use std::fs::File;
use std::io::{Result, Write};

#[derive(Debug)]
struct Document {
    name: String,
    content: Vec<String>,
    para_count: usize,
}

fn make_document(x: &str) -> Vec<String> {
    x.split("\n\n").map(|a| a.to_string()).collect()
}

fn rank_documents(x: &mut [Document]) {
    x.sort_by(|a, b| b.para_count.cmp(&a.para_count));
}

fn to_html<W: Write>(mut writer: W, ranked_docs: &[Document]) -> Result<()> {
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
    html.push_str("  <th>File</th>\n");
    html.push_str("  <th>Paragraph counts</th>\n");
    html.push_str("</tr>\n");

    for document in ranked_docs {
        let file_path = &document.name;
        let paragraph_count = document.para_count;

        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>\n", file_path, paragraph_count));
    }

    html.push_str("</table>");
    html.push_str("</body>");
    html.push_str("</html>");

    writer.write_all(html.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let file_paths = vec![
        "fox.txt",
        "bustle.txt",
        "para3.txt",
    ];

    let mut docs = Vec::new();

    for file_path in &file_paths {
        let file_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file {}: {}", file_path, err);
                continue;
            }
        };

        let content_lines: Vec<String> = file_content.split("\n\n").map(|a| a.to_string()).collect();
        let para_count = content_lines.len();

        let document = Document {
            name: file_path.to_string(),
            content: content_lines,
            para_count,
        };

        docs.push(document);
    }

    rank_documents(&mut docs);

    let output_file_path = "num_of_paragraphs.html";

    let mut html_file = File::create(output_file_path)?;

    to_html(&mut html_file, &docs)?;

    println!("HTML report saved to {}", output_file_path);
    Ok(())
}