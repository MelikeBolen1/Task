// Define the Account trait with required methods
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

// Implement methods for BankAccount
impl BankAccount {
    fn new(account_number: String, holder_name: String) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
        }
    }
}

// Implement the Account trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2} to account {}", amount, self.account_number);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= self.balance {
            self.balance -= amount;
            println!("Withdrawn ${:.2} from account {}", amount, self.account_number);
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
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

    // Perform some transactions
    account1.deposit(1000.0);
    if let Err(e) = account2.withdraw(500.0) {
        println!("Error: {}", e);
    }

    // Print final balances
    println!("Account {} balance: ${:.2}", account1.account_number, account1.balance());
    println!("Account {} balance: ${:.2}", account2.account_number, account2.balance());
} 