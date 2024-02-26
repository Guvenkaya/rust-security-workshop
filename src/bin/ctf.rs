use primitive_types::U256;

#[macro_use]
extern crate rocket;

struct User {
    name: String,
    balance: u128,
}

struct Item {
    name: String,
    price: u128,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::from("Jake"),
            balance: 321,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: String::from("Bored Ape"),
            price: 23,
        }
    }
}

fn fibonacci_calc(n: u128) -> u128 {
    match n {
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_calc(n - 1) + fibonacci_calc(n - 2),
    }
}

fn cast(num1: u128, num2: u128) -> u128 {
    let result = U256::from(num1).pow(U256::from(num2));

    result.as_u128()
}

// Try to spot issues without running the code first.
#[get("/fibonacci?<num>")]
fn fibonacci(num: Option<u128>) -> String {
    match num {
        Some(0) => String::from("Please enter a number bigger than 0"),
        Some(n) => {
            let res = fibonacci_calc(n);

            format!("Number {} is {}", n, res.to_string())
        }
        None => String::from("Please enter a number"),
    }
}

// Try to spot issues without running the code first.
#[get("/power?<num>&<num2>")]
fn power(num: Option<u128>, num2: Option<u128>) -> String {
    if num.is_some() && num2.is_some() {
        let res = cast(num.unwrap(), num2.unwrap());

        format!(
            "Result of the {} to power {} is {}",
            num.unwrap(),
            num2.unwrap(),
            res.to_string()
        )
    } else {
        String::from("Please enter all numbers")
    }
}

// Try to spot issues without running the code first.
#[get("/strings?<str>&<element>")]
fn split(str: Option<String>, element: Option<usize>) -> String {
    if str.is_some() {
        let str = str.unwrap();
        let element = element.unwrap();

        if str.len() < element {
            return String::from("Index is too high");
        }

        let slice = &str[..element];

        format!("The {} element of {} is {}", element, str, slice)
    } else {
        String::from("Please enter a string & element")
    }
}

// Try to spot issues without running the code first.
#[get("/auction?<bid>")]
fn auction(bid: Option<u128>) -> String {
    let mut user = User::default(); //every time you call this route, user resets
    let item = Item::default();

    match bid {
        Some(0) => String::from("Please enter a number larger than 0"),
        Some(n) => {
            if n <= user.balance && n >= item.price {
                let new_balance = user.balance - (n as u8) as u128;

                user.balance = new_balance;

                format!(
                    "You have purchased {} for {}. Your remaining balance is: {}",
                    item.name, n, user.balance
                )
            } else {
                format!(
                    "{}, not enough balance. Current Balance: {}. {} price: {}",
                    &user.name, &user.balance, &item.name, &item.price
                )
            }
        }
        None => String::from("Please enter a number"),
    }
}

#[get("/")]
fn greet() -> &'static str {
    "Can you spot all vulns?"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![greet, power, auction, split, fibonacci])
}
