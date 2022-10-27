use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    (weight / 9.81) * 3.711
}

// 1. Each value in Rust is owned by a variable.
// 2. When the owner goes out of scope, the value will be deallocated
// 3. There can only be one owner at a time