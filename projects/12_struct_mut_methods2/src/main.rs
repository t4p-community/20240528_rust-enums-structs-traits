struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Method to create a new BankAccount with an initial balance
    fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    // Method to deposit an amount to the account
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Method to withdraw an amount from the account
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err(String::from("Insufficient funds"))
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    // Method to check the current balance
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new(1000.0);

    println!("Initial balance: ${}", account.balance());

    account.deposit(500.0);
    println!("After deposit: ${}", account.balance());

    match account.withdraw(200.0) {
        Ok(()) => println!("After withdrawal: ${}", account.balance()),
        Err(e) => println!("Error: {}", e),
    }

    match account.withdraw(1500.0) {
        Ok(()) => println!("After withdrawal: ${}", account.balance()),
        Err(e) => println!("Error: {}", e),
    }
}
