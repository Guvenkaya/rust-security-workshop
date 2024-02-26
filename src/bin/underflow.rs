use std::io::stdin;

fn underflow(a: u8, b: u8) -> u8 {
    a - b
}

fn main() {
    println!("Enter any number");

    let mut number = String::new();
    stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let number = number
        .split_whitespace()
        .collect::<String>()
        .parse::<u8>()
        .unwrap();

    let result = underflow(number, 6);

    println!("{result}")
}
