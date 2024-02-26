fn divide_by_zero(a: u8, b: u8) -> u8 {
    a / b
}

fn main() {
    println!("{}", divide_by_zero(20, 0))
}
