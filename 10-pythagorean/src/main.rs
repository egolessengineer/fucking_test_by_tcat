use std::io;

fn main() {
    // println!("Hello, world!");
    let mut input_line = String::new();

    println!("Enter an integer N:");

    io::stdin().read_line(&mut input_line).unwrap();

    let n: u32 = match input_line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    find_pythagorean_triplets(n);
}

fn find_pythagorean_triplets(n: u32) {
    for a in 1..n {
        for b in a+1..n-a {
            let c = n - a- b;
            if a * a + b * b == c * c {
                println!("({}, {}, {})", a, b, c);
            }
        }
    }
}
