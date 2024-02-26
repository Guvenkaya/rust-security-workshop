use primitive_types::U256;

fn main() {
    // Using custom U256 type from primitive-types library
    let a = U256::MAX;

    // Panics on casting overflow
    println!("{}", a.as_u128());
}
