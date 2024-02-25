use csv::Reader;
use std::io;

fn minkowski_distance(x: Vec<f32>, y: Vec<Vec<f32>>, p: u32) -> f32 {
    assert_eq!(x.len(), y.len(), "Vectors must have the same length");

    let mut sum = 0.0;

    for (xi, yi) in x.iter().zip(y.iter()) {
        sum += f32::powf(xi.abs_sub(*yi), p as f32);
    }

    f32::powf(sum, 1.0 / (p as f32))
}



fn space_last_point(X:Vec<f32>, y:Vec<Vec<f32>> ,k: u32){
    let len = X.len();
    let last_point = &y[len-1];
    let mut total_sum = 0.0;
    for sub_vec in &y {
        for elem in sub_vec {
            total_sum += *elem;
        }
    }
    for sub_vec in &x {
        for elem in sub_vec {
            total_sum += *elem;
        }
    }
    for i in range (0, k){
        minkowski_distance();
    }
    for i in range (k, len - 2) {

    }
}

fn main(){
    let file_name =  "D:/RUST/Rust_model/KNN/k-neigbor/src/fake_data.csv";
    let reader = Reader::from_path(file_name);
    let mut X:Vec<f32> = Vec::new();
    let mut y:Vec<Vec<f32>> = Vec::new();
    if reader.is_err(){
        println!("Maybe file doesn't exist");
    }
    let mut my_reader = reader.unwrap();
    for records in my_reader.records(){
        let record = records.unwrap();
        let x_value: f32 = record.get(0).unwrap().parse().unwrap();
        let y_value_1: f32 = record.get(1).unwrap().parse().unwrap();
        let y_value_2: f32 = record.get(2).unwrap().parse().unwrap();
        X.push(x_value);
        y.push(vec![y_value_1,y_value_2]);
    }
    println!("Input your keys: ");
    let  mut key = String::new();
    io::stdin().read_line(&mut key).expect("failed to readline");
    let key: u32 = match key.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input must be an positve integer.");
            return;
        }
    };
    space_last_point(X,y,key);
}
