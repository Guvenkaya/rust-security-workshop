use std::fs::File;
use std::io::Error;
use std::path::Path;

//Main function now returns either Okay or throws an error.
fn main() -> Result<(), Error> {
    let name = "test.txt";

    let path = Path::new(name);

    //File::open returns Result<File, Error>,
    //so we can utilize ? to propagate the error
    let file = File::open(path)?;

    println!("{:?}", file.metadata());

    // We have to return Okay variant
    Ok(())
}
