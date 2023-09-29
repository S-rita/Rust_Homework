use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter};

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
    for i in 0..n {
        let name = format!("Layer {}", i + 1);
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


fn main() -> Result<(), Box<dyn Error>> {
    let n = 4;
    let mut rng = rand::thread_rng();
    let file = File::create("object_layer_list.csv")?;
    let buf_writer = BufWriter::new(file);
    let mut csv_writer = csv::Writer::from_writer(buf_writer);
    let layers = gen_obj_layer_list(&mut rng, n);

    for layer in &layers {
        for obj in &layer.objects {
            let mut record: Vec<String> = Vec::new();
            record.push(format!("{}", layer.name));
            record.push(layer.color.clone());
            record.push(obj.x.to_string());
            record.push(obj.y.to_string());
            record.push(obj.r.to_string());

            csv_writer.write_record(&record)?;
        }
    }
    csv_writer.flush()?;
    Ok(())
}
