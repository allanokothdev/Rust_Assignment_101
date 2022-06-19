
pub fn run() {

    //Initialize Vector Database to store Accounts
    let mut accounts: Vec<Account> = Vec::new();

    //Create and Add Account to the database
    println!("//CREATE NEW ACCOUNT//");
    let mut _allan_account = Account::new("allanokoth.testnet","Allan",1000.0);
    _allan_account.print_details();
    accounts.push(_allan_account);

    //Create and Add Account to the database
    println!("//CREATE NEW ACCOUNT//");
    let mut _james_account = Account::new("james.testnet","James",1000.0);
    _james_account.print_details();
    accounts.push(_james_account);

    //Update Account Name
    println!("//UPDATE ACCOUNT NAME//");
    accounts[0].update_account("Allan Okoth");
    println!("{:?}", accounts);

     //Depositing Money
    println!("//DEPOSIT 1000 & 2000 respectively//");
    accounts[0].deposit_money(1000.0);
    accounts[1].deposit_money(2000.0);
    println!("{:?}", accounts);
    

     //Withdrawing Money
     println!("//WITHDRAWING 500 & 700 respectively");
     accounts[0].withdraw_money(500.0);
     accounts[1].withdraw_money(700.0);
     println!("{:?}", accounts);

    //Transferring Money
    transfer_money(500.0, &mut accounts);
    println!("{:?}", accounts);

    //Function to Delete Account from the database
    println!("//DELETE ACCOUNT AT INDEX 1//");
    accounts.pop();
    println!("{:?}", accounts);

    //Function to Delete Account at Index 0
    println!("//DELETE ACCOUNT AT INDEX 0//");
    accounts.remove(0);
    println!("{:?}", accounts);

}

fn transfer_money(amount: f64, accounts:&mut Vec<Account>) {
    accounts[0].withdraw_money(amount);
    accounts[1].deposit_money(amount);
}


#[derive(Debug, Clone)]
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

}
trait AccountOperations {

    fn register(&mut self, account: &Account);
    
    fn update_account(&mut self, name: &str);

    fn delete_account(&mut self, account: &mut Vec<Account>);

    fn deposit_money(&mut self, amount: f64);

    fn withdraw_money(&mut self, amount: f64);

}