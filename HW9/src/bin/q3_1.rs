use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

#[derive(Debug, PartialEq)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}

fn gen_random_color<R: Rng>(rng: &mut R) -> String {
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    let a = rng.gen_range(0..=255);
    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

fn gen_random_circle<R: Rng>(rng: &mut R) -> Circle {
    let x = rng.gen_range(-100.0..=100.0);
    let y = rng.gen_range(-100.0..=100.0);
    let r = rng.gen_range(-10.0..=20.0);
    Circle { x, y, r }
}

fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: usize) -> Vec<Layer> {
    let mut layers = Vec::new();
    for i in 1..=n {
        let name = format!("Layer {:?}", i);
        let color = gen_random_color(rng);

        let mut objects = Vec::new();
        let n_objects = rng.gen_range(20..=50);

        for _ in 0..n_objects {
            objects.push(gen_random_circle(rng));
        }

        layers.push(Layer {
            name,
            color,
            objects,
        });
    }
    layers
}

fn average_to_html(layers: &[Layer], file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, th, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    html.push_str("<table width=30%>\n");

    html.push_str("<tr>\n");
    html.push_str("  <th>Name</th>\n");
    html.push_str("  <th>Average Area</th>\n");
    html.push_str("</tr>\n");

    for layer in layers {
        let average_area = calculate_average_area(&layer.objects);
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", layer.name));
        html.push_str(&format!("<td>{}</td>", average_area));
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>\n");

    file.write_all(html.as_bytes())?;

    Ok(())
}


fn calculate_average_area(circles: &[Circle]) -> f64 {
    let mut total_area = 0.0;
    const PI: f64 = 3.142;

    for circle in circles {
        let now_area = PI * circle.r.powi(2);
        total_area += now_area;
    }

    total_area / circles.len() as f64
}

fn read_csv(reader: impl Read) -> Result<Vec<Layer>, Box<dyn Error>> {
    let mut layer = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(reader);

    for result in rdr.records() {
        let record = result?;
        if record.len() < 5 {
            continue;
        }

        let name = record[0].to_string();
        let color = record[1].to_string();
        let x = f64::from_str(&record[2])?;
        let y = f64::from_str(&record[3])?;
        let r = f64::from_str(&record[4])?;

        let circle = Circle { x, y, r };
        let new_layer = Layer {
            name,
            color,
            objects: vec![circle],
        };

        layer.push(new_layer);
    }
    Ok(layer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let n = 4;

    let layers = gen_obj_layer_list(&mut rng, n);

    average_to_html(&layers, "average_areas.html")?;

    let input_csv_file = "object_layer_list.csv";
    let input_file = File::open(input_csv_file)?;
    read_csv(input_file)?;

    Ok(())
}