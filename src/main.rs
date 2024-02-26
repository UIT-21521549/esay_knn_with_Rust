use csv::Reader;
use std::io;

fn minkowski_distance(x: &Vec<f32>, y: &Vec<f32>, p: u32) -> f32 {
    assert_eq!(x.len(), y.len(), "Vectors must have the same length");

    let mut sum = 0.0;

    for (xi, yi) in x.iter().zip(y.iter()) {
        sum += f32::powf(xi.abs_sub(*yi), p as f32);
    }

    f32::powf(sum, 1.0 / (p as f32))
}

fn space_last_point(X: &Vec<f32>, y: &Vec<Vec<f32>>, k: usize) {
    let vec_for_first_k_X: Vec<f32> = X.iter().take(k).cloned().collect();
    let vec_for_first_k_y: Vec<&Vec<f32>> = y.iter().take(k).collect();
    let mut vec_for_first_k: Vec<f32> = Vec::new();

    for (i, sub_vec) in y.iter().enumerate().take(k) {
        let dist = minkowski_distance(&vec_for_first_k_X, sub_vec, 2);
        println!("Distance from {}th point: {}", i, dist);
        vec_for_first_k.push(dist);
    }
}

fn main() {
    let file_name = "D:/RUST/Rust_model/KNN/k-neigbor/src/fake_data.csv";
    let reader = Reader::from_path(file_name);
    let mut X: Vec<f32> = Vec::new();
    let mut y: Vec<Vec<f32>> = Vec::new();

    if let Err(e) = reader {
        println!("Error reading file: {}", e);
        return;
    }
    let mut my_reader = reader.unwrap();
    for record in my_reader.records() {
        let record = record.unwrap();
        let x_value: f32 = record.get(0).unwrap().parse().unwrap();
        let y_value_1: f32 = record.get(1).unwrap().parse().unwrap();
        let y_value_2: f32 = record.get(2).unwrap().parse().unwrap();
        X.push(x_value);
        y.push(vec![y_value_1, y_value_2]);
    }

    println!("Input your k: ");
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    let k: usize = match key.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input must be a positive integer.");
            return;
        }
    };

    if k >= X.len() {
        println!("k must be less than the number of data points.");
        return;
    }

    space_last_point(&X, &y, k);
}
