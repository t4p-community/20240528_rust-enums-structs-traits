use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    // This method is called when the struct needs to be formatted for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the write! macro to write the formatted string into the formatter.
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // have the effect of calling the fmt method with the formatter as argument.
    println!("{}", person);
}
