use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, PartialEq)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}

fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
    let mut avg_areas = Vec::new();

    for layer in layers {
        let mut total_area = 0.0;
        const PI: f64 = 3.14159265359;

        for circle in &layer.objects {
            let now_area = PI * circle.r.powi(2);
            total_area += now_area;
        }

        let average_area = total_area / layer.objects.len() as f64;
        avg_areas.push((layer.name.clone(), average_area));
    }

    avg_areas
}

fn read_layers_from_csv(file_name: &str) -> Result<Vec<Layer>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    let mut layers = Vec::new();
    let mut current_layer: Option<Layer> = None;

    for result in reader.records() {
        let record = result?;
        let mut iter = record.iter();

        let name = iter.next().unwrap().to_string();
        let color = iter.next().unwrap().to_string();
        let x = iter.next().unwrap().parse::<f64>()?;
        let y = iter.next().unwrap().parse::<f64>()?;
        let r = iter.next().unwrap().parse::<f64>()?;

        let circle = Circle { x, y, r };

        match &mut current_layer {
            Some(layer) if layer.name == name => {
                layer.objects.push(circle);
            }
            _ => {
                if let Some(layer) = current_layer.take() {
                    layers.push(layer);
                }
                current_layer = Some(Layer {
                    name: name.clone(),
                    color,
                    objects: vec![circle],
                });
            }
        }
    }

    if let Some(layer) = current_layer.take() {
        layers.push(layer);
    }

    Ok(layers)
}

fn write_average_areas_to_csv(file_name: &str, avg_areas: &[(String, f64)]) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_name)?;
    let buf_writer = BufWriter::new(file);
    let mut csv_writer = csv::Writer::from_writer(buf_writer);

    for (name, average_area) in avg_areas {
        csv_writer.write_record(&[name, &average_area.to_string()])?;
    }

    csv_writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "object_layer_list.csv";
    let layers_read = read_layers_from_csv(file_name)?;
    let avg_areas = cal_average_area(&layers_read);
    write_average_areas_to_csv("average_areas.csv", &avg_areas)?;

    Ok(())
}
