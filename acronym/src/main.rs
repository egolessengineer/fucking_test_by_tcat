use std::io;

fn main() {
    // println!("Hello, world!");
    let mut input_line = String::new();

    loop {
        input_line.clear();
        println!("Enter a string (or 'exit' to quit):");
        io::stdin().read_line(&mut input_line).unwrap();

        if input_line.trim().to_lowercase() == "exit" {
            println!("Exiting the program");
            break;
        }

        let acronym = generatee_acronym(&input_line);
        println!("{}", acronym);
    }
}

fn generatee_acronym(input: &str) -> String {
    input.split(|c:char| !c.is_alphabetic() && c != '-').filter_map(|word| {
        let trimmed_word = word.trim();
        if !trimmed_word.is_empty() {
            Some(trimmed_word.chars().next().unwrap().to_ascii_uppercase().to_string())
        } else {
            None
        }
    }).collect()
}