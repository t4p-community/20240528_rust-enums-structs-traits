struct Address {
    street: String,
    city: String,
    zip_code: String,
}

impl Address {
    fn full_address(&self) -> String {
        format!("{}, {}, {}", self.street, self.city, self.zip_code)
    }
}

struct Person {
    name: String,
    age: u32,
    address: Address,
}

impl Person {
    fn new(name: String, age: u32, street: String, city: String, zip_code: String) -> Person {
        Person {
            name,
            age,
            address: Address {
                street,
                city,
                zip_code,
            },
        }
    }

    fn introduce(&self) -> String {
        format!(
            "Hi, my name is {}. I am {} years old and I live at {}.",
            self.name,
            self.age,
            self.address.full_address()
        )
    }
}

fn main() {
    let person = Person::new(
        String::from("Alice"),
        30,
        String::from("123 Main St"),
        String::from("Springfield"),
        String::from("12345"),
    );

    println!("{}", person.introduce());
}
