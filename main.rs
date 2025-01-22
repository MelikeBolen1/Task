trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: String, holder_name: String) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two bank accounts
    let mut account1 = BankAccount::new(
        "1234567890".to_string(),
        "John Doe".to_string(),
    );
    let mut account2 = BankAccount::new(
        "0987654321".to_string(),
        "Jane Smith".to_string(),
    );

    // Test deposit with error handling
    match account1.deposit(1000.0) {
        Ok(()) => println!("Successfully deposited $1000.0 to account1"),
        Err(e) => println!("Error depositing to account1: {}", e),
    }

    // Test withdraw with error handling
    match account2.withdraw(500.0) {
        Ok(()) => println!("Successfully withdrew $500.0 from account2"),
        Err(e) => println!("Error withdrawing from account2: {}", e),
    }

    // Print final balances
    println!("Account1 balance: ${}", account1.balance());
    println!("Account2 balance: ${}", account2.balance());

    // Test error cases
    match account1.deposit(-100.0) {
        Ok(()) => println!("Successfully deposited -$100.0 to account1"),
        Err(e) => println!("Error depositing to account1: {}", e),
    }

    match account2.withdraw(1000.0) {
        Ok(()) => println!("Successfully withdrew $1000.0 from account2"),
        Err(e) => println!("Error withdrawing from account2: {}", e),
    }
} 