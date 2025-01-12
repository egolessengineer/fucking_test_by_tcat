use std::io;

fn main() {
    // println!("Hello, world!");
    let mut input_line = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input_line).unwrap();

    let n: u32 = match input_line.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("Please enter a positive integer.");
            return;
        }
    };

    let steps = collatz(n);
    println!("Number of steps: {}", steps);

}

fn collatz(n: u32) -> u32 {
    let mut steps = 0;
    let mut current = n;

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        steps += 1;
    }
    
    // println!("{}", current);
    steps

}


