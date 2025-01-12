use std::io;

fn main() {
    // println!("Hello, world!");
    let mut input_line = String::new();

    loop {
        input_line.clear();
        println!("Enter a number (or 0 to exit):");
        io::stdin().read_line(&mut input_line).unwrap();

        let number: i32 = input_line.trim().parse().unwrap_or(0);
        println!("{}", is_armstrong_number(number));
    }
}

fn is_armstrong_number(num: i32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let sum: i32 = num_str.chars().map(|c| c.to_digit(10).unwrap().pow(num_digits) as i32).sum(); 
    sum == num
}