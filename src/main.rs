use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};

pub mod Knn;
use crate::Knn::*;

fn read_csv(file_name: &str) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    
    let mut X: Vec<Vec<f32>> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut first_line_skipped = false;
    for line in reader.lines() {
        let record = line?;
        // Bỏ qua dòng đầu tiên trong file CSV
        if !first_line_skipped {
            first_line_skipped = true;
            continue;
        }
        // Tách các phần từ trong dòng thành mảng
        let parts: Vec<&str> = record.split(',').collect();

        // Chuyển đổi các phần từ sang dạng số thực
        let mut x_values: Vec<f32> = Vec::new();
        for part in &parts[..(parts.len() - 1)] {
            let value: f32 = part.parse().expect("Failed to parse number");
            x_values.push(value);
            
        }
        let y_value: f32 = parts[parts.len() - 1].parse().expect("Failed to parse number");

        // Thêm dữ liệu X và y vào các vector tương ứng
        X.push(x_values);
        y.push(y_value);
    }

    // Thêm vector y vào cuối vector X để tạo thành một vector duy nhất chứa toàn bộ dữ liệu
    X.push(y);
    
    // Trả về vector chứa dữ liệu đã đọc được từ file CSV
    Ok(X)
}


fn main() {
    let file_name = "D:/Rust/esay_knn_with_Rust/src/fake_data.csv";
    let f_data = read_csv(file_name);
    match f_data {
        Ok(data)=>{
            let mut x: Vec<Vec<f32>> = Vec::new();
            let len = data.len();
            let X: Vec<Vec<f32>> = data[0..(len - 1)].iter().map(|v| v.clone()).collect();
            let Y: Vec<i32> = data[len - 1].iter().map(|v| *v as i32).collect();
            let mut model = Model::init(X,Y);
            let x: Vec<f32> = vec![0.9, 0.4];
            println!("{}",model.k_neiborgh_nearest(3, &x));
        },
        Err(err)=>{
            println!("{}",err)
        }
    }
}
