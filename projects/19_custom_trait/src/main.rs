// Define the Summary trait with a default implementation
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement the Summary trait for the Article struct with a custom implementation
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

// Implement the Summary trait for the Tweet struct without a custom implementation
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {}

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

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
}
