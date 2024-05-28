use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
enum CustomError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Io(ref err) => write!(f, "IO error: {}", err),
            CustomError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<ParseIntError> for CustomError {
    fn from(err: ParseIntError) -> CustomError {
        CustomError::Parse(err)
    }
}

fn read_and_parse_file(filename: &str) -> Result<i32, CustomError> {

    // if the file doesn't exist, this will return an io::Error
    // the io::Error will be converted into a CustomError::Io
    let mut file = File::open(filename)?; 
    let mut contents = String::new();

    // if the file can't be read, this will return an io::Error
    // the io::Error will be converted into a CustomError::Io
    file.read_to_string(&mut contents)?; 

    // if the string can't be parsed into an integer,
    // this will return a ParseIntError
    // the ParseIntError will be converted into a CustomError::Parse
    let number: i32 = contents.trim().parse()?; 

    Ok(number)
}

fn main() {
    match read_and_parse_file("example.txt") {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}