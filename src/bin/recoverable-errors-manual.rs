use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
    let name = "test.txt";

    let path = Path::new(name);
    let display = path.display();

    //File::open returns Result<File, Error>

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("File: {display} cannot be opened. Error: {e}");
            process::exit(1);
        }
    };

    println!("{:?}", file.metadata());
}