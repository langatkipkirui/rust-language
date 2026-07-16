// STRUCT: A data structure that allows you to group multiple fields together under one name

pub fn run() {
    let mut account: BankAccount = BankAccount { owner: "Kevin".to_string(), balance: 250.2 };

    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw an amount
    account.withdraw(50.5);

    // checking the account again
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
}
