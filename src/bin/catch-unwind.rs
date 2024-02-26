fn divide_by_zero(a: u8, b: u8) -> u8 {
    a / b
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        let _ = divide_by_zero(100, 0);
    });

    match result {
        Ok(_) => println!("No panic occurred"),
        Err(_) => println!("Panic occurred"),
    }
}
