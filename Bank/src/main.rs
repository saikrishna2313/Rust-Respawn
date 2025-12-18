#[derive(Debug)]
struct Account {
    id: u32,
    user: String,
    balance: i32,
}

impl Account {
    fn new(id: u32, user: impl Into<String>) -> Self {
        Self {
            id,
            user: user.into(),
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn summary(&self) -> String {
        format!("Account {} ({}) → balance {}", self.id, self.user, self.balance)
    }
}

#[derive(Debug, Default)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Self::default()
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|a| a.balance).sum()
    }

    fn accounts_summary(&self) -> Vec<String> {
        self.accounts.iter().map(Account::summary).collect()
    }
}

fn main() {
    let mut bank = Bank::new();

    // Create multiple accounts
    let mut acc1 = Account::new(1, "Varma");
    let mut acc2 = Account::new(2, "Alice");
    let mut acc3 = Account::new(3, "Bob");
    
    
    // Perform transactions
    acc1.deposit(100);
    acc1.withdraw(20);

    println!("{}",acc1.summary());

    acc2.deposit(200);
    acc2.withdraw(50);
    
    println!("{}",acc1.summary());

    acc3.deposit(500);
    println!("{}",acc1.summary());
    // Add accounts to the bank
    bank.add_account(acc1);
    bank.add_account(acc2);
    bank.add_account(acc3);
    
    println!("{:#?}",bank.accounts_summary());

    for summary in bank.accounts_summary() {
        println!("{}", summary);
    }

    println!("\nTotal Bank Balance: {}", bank.total_balance());
}
