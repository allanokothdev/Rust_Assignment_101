#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: f64,
}

impl Account {

    pub fn new(id: &str, name: &str, balance: f64) -> Self {
        Self { id: id.to_string(), name: name.to_string(), balance: balance }
    }
}

pub fn transfer_money(amount: f64, accounts:&mut Vec<Account>) {
    accounts[0].withdraw_money(amount);
    accounts[1].deposit_money(amount);
}


impl AccountOperations for Account {

    //UPDATE Account operation
    fn update_account(&mut self, name: &str) {
        self.name = name.to_string();
    }

    //DELETE Account operation
    fn delete_account(&mut self, accounts:&mut Vec<Account>) {
        accounts.pop();
    }

    //DEPOSIT MONEY operation
    fn deposit_money(&mut self, amount: f64) {
        self.balance += amount;
    }

    //WITHDRAWING MONEY operation
    fn withdraw_money(&mut self, amount: f64) {
        self.balance -= amount;
    }

}

pub trait AccountOperations {

    fn update_account(&mut self, name: &str);

    fn delete_account(&mut self, account: &mut Vec<Account>);

    fn deposit_money(&mut self, amount: f64);

    fn withdraw_money(&mut self, amount: f64);

}