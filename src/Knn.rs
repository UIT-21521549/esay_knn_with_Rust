// Weight is weight for predict, caculated by processing train
//x,y : is input
pub struct Model{
    x: Vec<Vec<f32>>,
    y: Vec<i32>
}

// //sigmoid for binary classfication
// // y is predict from model
// //if y -> extremely => sigmoid -> 1 oppsite y -> negative extremely => sigmoid -> 0 
// fn sigmoid(y: f32) -> f32{
//     let e: f32 = 2.71828;
//     1.0 / (1.0 + e.powf(-y))
// }

//get space of vector
fn minkowski_distance(x: &Vec<f32>, y: &Vec<f32>, p: i32) -> f32 {
    assert_eq!(x.len(), y.len(), "Vectors must have the same length");

    let mut sum = 0.0;
    let len = x.len() as f32;
    for (xi, yi) in x.iter().zip(y.iter()) {
        sum += f32::powf((xi - (*yi as f32)).abs(),p as f32);
    }
    sum = f32::powf(sum, 1.0/p as f32);
    sum
}

// i design model based gradient descent
impl Model{

    //init model
    pub fn init(X: Vec<Vec<f32>>, y: Vec<i32>) -> Model{
        Model{x: X, y: y}
    }

    //space of points with last point
    pub fn k_neiborgh_nearest(&mut self, k: i32, x: &Vec<f32>) -> i32{
        let mut space:Vec<Vec<f32>> = Vec::new();
        for i in 0..self.x.len(){
            let p = self.x[0].len() as i32;
            let mut a: Vec<f32> = Vec::new();
            let s = i as f32;
            a.push(minkowski_distance(&self.x[i],&x, p));
            a.push(s);
            space.push(a);
        }
        //sort by space vector
        space.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap_or(std::cmp::Ordering::Equal));
        let mut class_0 = 0;
        let mut class_1 = 0;
        for i in 0..k {
            let j: usize = space[i as usize][1].round() as usize;
            if (self.y[j] == 1){
                class_1 +=1;
            }else{
                class_0 += 0;
            }
        }
        //return index of data point that is most like current point
        if  class_1 > class_0 {
            return 1;
        }
        else if class_1 < class_0{
            return 0;
        }
        let k = k as usize;
        let j = space[k][1].round() as usize;
        return self.y[j];

    }

}
