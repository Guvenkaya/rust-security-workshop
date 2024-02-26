fn factorial_calc(num: u128) -> u128 {
    if num > 0 {
        if num <= 1 {
            return 1;
        }
        num.saturating_mul(factorial_calc(num - 1))
    } else {
        0
    }
}

fn main() {
    println!("{}", factorial_calc(100000))
}