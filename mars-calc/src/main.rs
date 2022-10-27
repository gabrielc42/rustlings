use std::io;

fn main() {
    // let s1 = &input;
    // let s2 = &input;
    // println!("{} {}", s1, s2);

    // some_fn(&mut input);

    println!("Enter your weight in kilograms: ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    // mars_weight = mars_weight * 1000.0; in grams
    println!("Weight on Mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    (weight / 9.81) * 3.711
}

// 1. Each value in Rust is owned by a variable.
// 2. When the owner goes out of scope, the value will be deallocated
// 3. There can only be one owner at a time

// a single mutable borrow or as many immutable borrows

// fn some_fn(s: &mut String) {
//     s.push_str("a");
// }