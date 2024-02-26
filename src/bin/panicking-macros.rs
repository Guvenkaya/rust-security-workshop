use std::io::stdin;

// No num || Not integer -> Panic unwrap
// Odd Number -> Panic on panic!
// Num > 10000 -> Panic on unimplemented!
// Num != secret_num -> Panic on assert!

fn main() {
    println!("Enter any number");

    let secret_number = 28;

    let mut number = String::new();
    stdin().read_line(&mut number).expect("Failed to read input");

    let number = number.split_whitespace().collect::<String>().parse::<i32>().unwrap();

    if number % 2 != 0 {
        panic!("Only even numbers are accepted");
    }

    if number > 10000 {
        unimplemented!();
    }

    assert!(number == secret_number, "You picked wrong number");

    println!("Entered number is {number}");
}