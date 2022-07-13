use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

pub fn run() -> Result<(), Box<dyn Error>> {
    // let condition = true;

    return Err(Box::new(MyError("Oops".into())));
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e); // "There is an error: Oops"
    }
}