use std::io;
use std::f64::consts::PI;

fn main() {
    // println!("Hello, world!");
    let mut input_line = String::new();

    println!("Enter the width and height of the rectangle seperated by a space:");

    io::stdin().read_line(&mut input_line).unwrap();

    let dimensions: Vec<f64> = input_line.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if dimensions.len() != 2 {
        println!("Please enter exactly two numbers.");
        return;
    }

    let (width, height) = (dimensions[0], dimensions[1]);
    println!("{} {}", calculate_area_and_perimeter((width, height)).0,calculate_area_and_perimeter((width, height)).1);

}   

fn calculate_area_and_perimeter(dimensions: (f64, f64)) -> (f64, f64) {
   let (width, height) = dimensions;
   let diameter:f64; 
   if width <= height {
    diameter = width;
   } else {
    diameter = height;
   }
   let radius = diameter / 2.0;

   let area = PI * radius * radius;
   let perimeter = 2.0 * PI * radius;

   (area, perimeter)
}
