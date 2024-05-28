struct Counter {
    value: i32,
}

impl Counter {
    // Method to create a new Counter with an initial value
    fn new(initial: i32) -> Counter {
        Counter { value: initial }
    }

    // Method to increment the value of the counter
    fn increment(&mut self) {
        self.value += 1;
    }

    // Method to reset the value of the counter to zero
    fn reset(&mut self) {
        self.value = 0;
    }

    // Method to get the current value of the counter
    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut counter = Counter::new(5);

    println!("Initial value: {}", counter.value());

    counter.increment();
    println!("After increment: {}", counter.value());

    counter.reset();
    println!("After reset: {}", counter.value());
}
