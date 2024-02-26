use std::io::Error;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::from("Jake"),
            age: 34,
        }
    }
}

fn main() {
    let a: Option<u128> = Some(2);
    let b: Option<u128> = None;
    let c: Option<Option<u128>> = Some(None);
    let d: Option<Option<u128>> = None;
    let e: Result<u128, Error> = Ok(2);
    let f: Result<u128, Error> = Err(Error::from_raw_os_error(2));
    let g: Result<Option<u128>, Error> = Ok(Some(2));
    let h: Result<Option<u128>, Error> = Ok(None);
    let i: Option<User> = Some(User {
        name: String::from("Jane"),
        age: 22,
    });
    let j: Option<User> = None;
    let k: Result<User, Error> = Ok(User {
        name: String::from("Jane"),
        age: 22,
    });
    let l: Result<User, Error> = Err(Error::from_raw_os_error(2));

    println!("a: {:?}", a.unwrap_or_default());
    println!("b: {:?}", b.unwrap_or_default());
    println!("c: {:?}", c.unwrap_or_default());
    println!("d: {:?}", d.unwrap_or_default());
    println!("e: {:?}", e.unwrap_or_default());
    println!("f: {:?}", f.unwrap_or_default());
    println!("g: {:?}", g.unwrap_or_default());
    println!("h: {:?}", h.unwrap_or_default());
    println!("i: {:?}", i.unwrap_or_default());
    println!("j: {:?}", j.unwrap_or_default());
    println!("k: {:?}", k.unwrap_or_default());
    println!("l: {:?}", l.unwrap_or_default());
}
