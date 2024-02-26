const PER_DAY: u128 = 100;

const FEE: u128 = 13;

pub fn get_rewards_right(amount: u128, start_date: u128, end_date: u128) -> u128 {
    (amount * PER_DAY * FEE) / (end_date - start_date)
}

pub fn get_rewards_wrong(amount: u128, start_date: u128, end_date: u128) -> u128 {
    (amount / (end_date - start_date)) * PER_DAY * FEE
}

pub fn get_rewards_wrong_float(amount: f64, start_date: f64, end_date: f64) -> f64 {
    (amount / (end_date - start_date)) * (PER_DAY as f64) * (FEE as f64)
}

pub fn get_rewards_wrong_float_convert(amount: f64, start_date: f64, end_date: f64) -> u128 {
    ((amount / (end_date - start_date)) * (PER_DAY as f64) * (FEE as f64)) as u128
}

fn main() {
    // 30 * 100 * 13 = 39000
    // 39000 / 13 = 3000
    let my_rewards_correct = get_rewards_right(30, 12, 25);

    // 30 / 13 => ~2.3076 is floored to 2
    // 2 * 100 * 13 = 2600
    let my_rewards_wrong = get_rewards_wrong(30, 12, 25);

    //(30.0/13.0) *100.0*13.0 = 2999.9999999999995
    let my_rewards_wrong_float = get_rewards_wrong_float(30.0, 12.0, 25.0);

    //(30.0/13.0) *100.0*13.0 = 2999.9999999999995 => 2999
    let my_rewards_wrong_float_convert = get_rewards_wrong_float_convert(30.0, 12.0, 25.0);

    println!("Correct: {my_rewards_correct}");
    println!("Wrong: {my_rewards_wrong}");
    println!("Wrong Float: {my_rewards_wrong_float}");
    println!("Wrong Float Convert: {my_rewards_wrong_float_convert}")
}