use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_and_parse_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(filename)?; // This can return a std::io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // This can return a std::io::Error

    let number: i32 = contents.trim().parse()?; // This can return a std::num::ParseIntError

    Ok(number)
}

fn main() {
    match read_and_parse_file("example.txt") {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}
