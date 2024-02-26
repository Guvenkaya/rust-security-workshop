const FEE: f32 = 0.075;
const PER_DAY: u16 = 2;
pub fn get_rewards(days: u16) -> f32 {
    (days * PER_DAY) as f32 * FEE
}

fn main() {
    // 1.5 rounded to 2
    let my_rewards_round = get_rewards(10).round();

    // 1.5 rounded to 2
    let my_rewards_ceil = get_rewards(10).ceil();

    // 1.5 rounded to 1
    let my_rewards_floor = get_rewards(10).floor();

    println!("Rewards rounded {my_rewards_round}");
    println!("Rewards ceiled {my_rewards_ceil}");
    println!("Rewards floored {my_rewards_floor}")
}
