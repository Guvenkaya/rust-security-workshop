fn main() {
    let a: u16 = 300;

    // Returns 44 while silently overflowing.
    println!("{}", a as u8)
}
