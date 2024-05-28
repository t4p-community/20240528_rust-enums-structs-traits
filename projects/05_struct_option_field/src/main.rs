struct User {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

fn main() {
    let user1 = User {
        first_name: String::from("John"),
        middle_name: Some(String::from("Paul")),
        last_name: String::from("Doe"),
    };

    let user2 = User {
        first_name: String::from("Jane"),
        middle_name: None,
        last_name: String::from("Smith"),
    };

    print_user_name(&user1);
    print_user_name(&user2);
}

fn print_user_name(user: &User) {
    match &user.middle_name {
        Some(middle_name) => println!("Full name: {} {} {}", user.first_name, middle_name, user.last_name),
        None => println!("Full name: {} {}", user.first_name, user.last_name),
    }
}
