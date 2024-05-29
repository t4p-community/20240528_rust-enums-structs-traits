struct Person {
    name: String,
    age: u32,
}

impl From<(String, u32)> for Person {
    fn from(tuple: (String, u32)) -> Self {
        Person {
            name: tuple.0,
            age: tuple.1,
        }
    }
}

fn main() {
    // Using the From trait explicitly
    let person_tuple = (String::from("Alice"), 30);
    let person = Person::from(person_tuple);

    println!("Using From - Name: {}, Age: {}", person.name, person.age);

    // Using the Into trait implicitly
    let another_person_tuple = (String::from("Bob"), 25);
    let another_person: Person = another_person_tuple.into();

    println!("Using Into - Name: {}, Age: {}", another_person.name, another_person.age);
}
