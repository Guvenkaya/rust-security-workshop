use std::process;

fn underflow_checked(a: u8, b: u8) -> Option<u8> {
    a.checked_sub(b)
}

fn overflow_saturating(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

fn main() {
    // Checked maths used. If None returned -> Underflow
    let val = underflow_checked(5, 6);

    // Saturating maths used. In case of addition/multiplication
    // -> u8::MAX is returned
    let val2 = overflow_saturating(250, 10);

    println!("{val2}");

    //Handling None case of checked maths without panicking.
    if val.is_none() {
        print!("Underflow occurred. Please try again");
        process::exit(1);
    }
    // We already handled underflow case -> unwrap is safe
    println!("{}", val.unwrap())
}
