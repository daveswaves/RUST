/*
Rust Programming Full Course (BekBrace) : 3 hours 5 mins
https://youtu.be/rQ_J9WH6CGk?t=5223
*/

fn main() {
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.95
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(49.95);

    // Recheck balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing £{:.2} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of £{:.2}", self.owner, self.balance);
    }
}


//=================================

fn main() {
    let mut x: u32 = 5;
    let y: &mut u32 = &mut x;
    *y += 1;
    // println!("x: {}", x);
    println!("y: {}", y);
}

