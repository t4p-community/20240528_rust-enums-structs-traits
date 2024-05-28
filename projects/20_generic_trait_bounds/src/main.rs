trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn print_summary<T: Summary>(item: T) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Rust Programming"),
        author: String::from("Jane Doe"),
        content: String::from("Rust is a systems programming language..."),
    };

    let tweet = Tweet {
        username: String::from("@rustacean"),
        content: String::from("Rust is amazing! #rustlang"),
    };

    print_summary(article);
    print_summary(tweet);
}
