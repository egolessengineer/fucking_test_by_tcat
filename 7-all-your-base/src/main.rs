use std::io;

fn main() {
    let mut input_line = String::new();

    loop {
        input_line.clear();
        println!("Enter a number and its base (or 0 to exit):");
        io::stdin().read_line(&mut input_line).unwrap();

        let inputs: Vec<i32> = input_line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if inputs.len() != 2 {
            println!("Please enter exactly two numbers.");
            return;
        }

        let (number, base) = (inputs[0], inputs[1]);
        let decimal = convert_to_decimal(number, base);
        println!("{}", decimal);
    }
}

fn convert_to_decimal(number: i32, base: i32) -> i32 {
    let mut num = number;
    let mut decimal = 0;
    let mut power = 0;

    while num > 0 {
        let digit = num % 10;
        decimal += digit * base.pow(power);
        power += 1;
        num /= 10;
    }

    decimal
}