use std::io;
use rand::Rng;

fn main() {
    let mut input_line = String::new();

    loop {
        input_line.clear();
        println!("Enter the numbers (or 0 0 to exit):");
        io::stdin().read_line(&mut input_line).unwrap();

        let nums: Vec<i32> = input_line.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

        if nums.len() != 2 {
            println!("Please enter exactly two numbers.");
            continue;
        }

        let (a, b) = (nums[0], nums[1]);

        if a == 0 && b == 0 {
            break;
        }

        let lower = a.min(b);
        let upper = a.max(b);
        let random_num = rand::thread_rng().gen_range(lower..=upper);

        println!("Random number between {} and {} is: {}", lower, upper, random_num);
    }

    println!("Exiting the program")
}
