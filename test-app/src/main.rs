use std::io;

fn main() {
    println!("Enter weight");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("weight on Mars: {}kg",mars_weight)
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}