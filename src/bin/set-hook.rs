fn divide_by_zero(a: u8, b: u8) -> u8 {
    a / b
}

fn main() {
    std::panic::set_hook(Box::new(|_| {
        println!("Some error");
    }));

    let _ = std::panic::catch_unwind(|| {
        let _ = divide_by_zero(100, 0);
    });

    let _ = std::panic::take_hook();

    panic!("This is a panic");
}
