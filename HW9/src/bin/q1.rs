use rand::Rng;

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

//////////// 1.1 ////////////
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

#[test]
fn test_gen_obj_layer_list() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);

    assert_eq!(layers.len(), n);

    for (i, layer) in layers.iter().enumerate() {
        assert_eq!(layer.name, format!("Layer {}", i + 1));
        assert_eq!(layer.color.len(), 9);

        let n_objects = layer.objects.len();
        assert!(n_objects >= 20 && n_objects <= 50);

        for circle in &layer.objects {
            assert!(circle.x >= -100.0 && circle.x <= 100.0);
            assert!(circle.y >= -100.0 && circle.y <= 100.0);
            assert!(circle.r >= -10.0 && circle.r <= 20.0);
        }
    }
}


//////////// 1.2 ////////////
fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
    let mut avg_areas = Vec::new();

    for layer in layers {
        let mut total_area = 0.0;
        const PI: f64 = 3.142;

        for circle in &layer.objects {
            let now_area = PI * circle.r.powi(2);
            total_area += now_area;
        }

        let average_area = total_area / layer.objects.len() as f64;
        avg_areas.push((layer.name.clone(), average_area));
    }

    avg_areas
}

#[test]
fn test_cal_average_area() {
    let layers = vec![
        Layer {
            name: "Layer 1".to_string(),
            color: "#RRGGBBAA".to_string(),
            objects: vec![
                Circle { x: 0.0, y: 0.0, r: 5.0,},
                Circle { x: 0.0, y: 0.0, r: 10.0,},
            ],
        },
        Layer {
            name: "Layer 2".to_string(),
            color: "#RRGGBBAA".to_string(),
            objects: vec![
                Circle { x: 0.0, y: 0.0, r: 15.0,
                },
                Circle { x: 0.0, y: 0.0, r: 25.0,
                },
                Circle { x: 0.0, y: 0.0, r: 35.0,
                },
            ],
        },
    ];

    let avg_areas = cal_average_area(&layers);
    assert_eq!(avg_areas.len(), 2);

    assert_eq!(
        avg_areas[0],
        ("Layer 1".to_string(), 392.5)
    );
    assert_eq!(
        avg_areas[1],
        ("Layer 2".to_string(), 4343.666666666667)
    );
}

fn main() {}
