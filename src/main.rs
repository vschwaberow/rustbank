use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
enum TransactionError {
    InsufficientFunds,
    AccountNotFound,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionError::InsufficientFunds => write!(f, "Insufficient funds"),
            TransactionError::AccountNotFound => write!(f, "Account not found"),
        }
    }
}

impl Error for TransactionError {}

struct Bank {
    accounts: HashMap<String, f64>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, name: String, initial_balance: f64) {
        self.accounts.insert(name, initial_balance);
    }

    fn transaction(&mut self, from: &str, to: &str, amount: f64) -> Result<(), TransactionError> {
        let from_balance = self.accounts.get(from).cloned();
        let to_balance = self.accounts.get(to).cloned();

        match (from_balance, to_balance) {
            (Some(from_balance), Some(to_balance)) => {
                if from_balance >= amount {
                    self.accounts
                        .insert(from.to_string(), from_balance - amount);
                    self.accounts.insert(to.to_string(), to_balance + amount);
                    Ok(())
                } else {
                    Err(TransactionError::InsufficientFunds)
                }
            }
            _ => Err(TransactionError::AccountNotFound),
        }
    }

    fn balance(&self, account: &str) -> Option<f64> {
        self.accounts.get(account).cloned()
    }
}

fn main() {
    let mut bank = Bank::new();

    loop {
        println!("Banking System Menu:");
        println!("1. Create Account");
        println!("2. Perform Transaction");
        println!("3. Check Balance");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => create_account(&mut bank),
            2 => perform_transaction(&mut bank),
            3 => check_balance(&mut bank),
            4 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn create_account(bank: &mut Bank) {
    println!("Enter account name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = name.trim().to_string();

    println!("Enter initial balance:");
    let mut balance = String::new();
    io::stdin()
        .read_line(&mut balance)
        .expect("Failed to read input");
    let balance: f64 = match balance.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid balance. Please try again.");
            return;
        }
    };

    bank.create_account(name, balance);
    println!("Account created successfully!");
}

fn perform_transaction(bank: &mut Bank) {
    println!("Enter source account name:");
    let mut from = String::new();
    io::stdin()
        .read_line(&mut from)
        .expect("Failed to read input");
    let from = from.trim();

    println!("Enter destination account name:");
    let mut to = String::new();
    io::stdin()
        .read_line(&mut to)
        .expect("Failed to read input");
    let to = to.trim();

    println!("Enter transaction amount:");
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount: f64 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount. Please try again.");
            return;
        }
    };

    match bank.transaction(from, to, amount) {
        Ok(_) => println!("Transaction successful!"),
        Err(err) => println!("Error: {}", err),
    }
}

fn check_balance(bank: &Bank) {
    println!("Enter account name to check balance:");
    let mut account = String::new();
    io::stdin()
        .read_line(&mut account)
        .expect("Failed to read input");
    let account = account.trim();

    if let Some(balance) = bank.balance(account) {
        println!("Balance of {}: {}", account, balance);
    } else {
        println!("Account not found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let mut bank = Bank::new();
        bank.create_account("Alice".to_string(), 100.0);
        assert_eq!(bank.accounts.get("Alice").unwrap(), &100.0);
    }

    #[test]
    fn test_transaction_success() {
        let mut bank = Bank::new();
        bank.create_account("Alice".to_string(), 100.0);
        bank.create_account("Bob".to_string(), 50.0);
        assert!(bank.transaction("Alice", "Bob", 20.0).is_ok());
        assert_eq!(bank.accounts.get("Alice").unwrap(), &80.0);
        assert_eq!(bank.accounts.get("Bob").unwrap(), &70.0);
    }

    #[test]
    fn test_transaction_insufficient_funds() {
        let mut bank = Bank::new();
        bank.create_account("Alice".to_string(), 100.0);
        bank.create_account("Bob".to_string(), 50.0);
        assert!(bank.transaction("Alice", "Bob", 120.0).is_err());
        assert_eq!(bank.accounts.get("Alice").unwrap(), &100.0);
        assert_eq!(bank.accounts.get("Bob").unwrap(), &50.0);
    }

    #[test]
    fn test_transaction_account_not_found() {
        let mut bank = Bank::new();
        bank.create_account("Alice".to_string(), 100.0);
        assert!(bank.transaction("Alice", "Charlie", 20.0).is_err());
        assert_eq!(bank.accounts.get("Alice").unwrap(), &100.0);
    }
}
