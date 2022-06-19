
pub fn run() {

    //Initialize Vector Database to store Accounts
    let mut accounts: Vec<Account> = Vec::new();

    //Create and Add Account to the database
    println!("//CREATE NEW ACCOUNT//");
    let mut _allan_account = Account::new("allanokoth.testnet","Allan",100.0);
    _allan_account.print_details();
    accounts.push(_allan_account);

    //Create and Add Account to the database
    println!("//CREATE NEW ACCOUNT//");
    let mut _james_account = Account::new("james.testnet","James",100.0);
    _james_account.print_details();
    accounts.push(_james_account);

     //Depositing Money
    println!("//DEPOSIT MONEY//");
    accounts[0].deposit_money(50.0);
    accounts[1].deposit_money(50.0);

     //Withdrawing Money
     println!("//WITHDRAWING MONEY");
     accounts[0].withdraw_money(200.0);
     accounts[1].withdraw_money(100.0);

    //Transferring Money
    println!("//MONEY TRANSFER//");
    accounts[0].transfer_money(150.0, &mut accounts[1]);

    //Function to Delete Account at Index 0
    println!("//DELETE ACCOUNT AT INDEX 0//");
    accounts.pop();

}


#[derive(Debug)]
struct Account {
    id: String,
    name: String,
    balance: f64,
}

impl Account {
    pub fn new(id: &str, name: &str, balance: f64) -> Self {
        Account {id: id.to_string(), name: name.to_string(), balance: balance}
    }

    pub fn print_details(&self) {
        println!("{:?}", self);
    }
}

impl AccountOperations for Account {

    fn register(&mut self, account: &Account) {
        println!("{:?}", account);
    }

    fn update_account(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn delete_account(&mut self, accounts:&mut Vec<Account>) {
        accounts.pop();
    }

    fn deposit_money(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw_money(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn transfer_money(&mut self, amount: f64, target_account: &mut Account) {
        self.withdraw_money(amount);
        target_account.balance += amount;
    }

}
trait AccountOperations {

    fn register(&mut self, account: &Account);
    
    fn update_account(&mut self, name: &str);

    fn delete_account(&mut self, account: &mut Vec<Account>);

    fn deposit_money(&mut self, amount: f64);

    fn withdraw_money(&mut self, amount: f64);

    fn transfer_money(&mut self, amount: f64, target_account: &mut Account);

}