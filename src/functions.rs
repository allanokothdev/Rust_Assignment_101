pub fn bank() {

    //Initialize Vector Database to store Accounts
    let mut accounts: Vec<Account> = Vec::new();

    //Create and Add Account to the database
    println!("//CREATE NEW ACCOUNT//");
    let allan = accounts.push(Account::new("Allan", 100.0));
    println!("{:?}", allan);
    println!("{:?}", accounts[0]);

    println!("//CREATE NEW ACCOUNT//");
    let bob = accounts.push(Account::new("Bob", 100.0));
    println!("{:?}", bob);
    println!("{:?}", accounts[1]);

    println!("//FECTH ALL RECORDS//");
    //Print all records
    println!("{:?}", accounts);

    println!("//CHANGE NAME OF ACCOUNT at INDEX 0//");
    //Fetch and change Name of an Account at Index 0
    let new_name = "Okoth";
    accounts[0].name = new_name.to_string();
    println!("I changed my name to {}",accounts[0].name);

    //Depositing Money
    println!("//DEPOSIT MONEY//");
    let pre_deposit_balance = accounts[0].balance;
    let deposit = 100.0;
    accounts[0].balance = pre_deposit_balance + deposit;
    println!("{} + {} = {}", pre_deposit_balance, deposit, accounts[0].balance);

    //Withdrawing Money
    println!("//WITHDRAWING MONEY");
    let withdraw = 100.0;
    let pre_withdrawal_balance = accounts[0].balance;
    if pre_withdrawal_balance >= withdraw {
        accounts[0].balance = pre_withdrawal_balance - withdraw;
        println!("{} - {} = {}", pre_withdrawal_balance, withdraw, accounts[0].balance);
    } else {
        println!("Insufficient Balance");
    }

    //Transferring Money
    println!("//MONEY TRANSFER//");
    let pre_transfer_sender_balance = accounts[0].balance;
    let pre_transfer_recipient_balance = accounts[1].balance;
    println!("Sender Balance: {:?}", pre_transfer_sender_balance);
    println!("Recipient Balance: {:?}", pre_transfer_recipient_balance);
    let transfer = 50.0;

    if pre_transfer_sender_balance >= transfer {
        accounts[0].balance = pre_transfer_sender_balance - transfer;
        accounts[1].balance = pre_transfer_recipient_balance + transfer;
        println!(" Transferring {} to Bob...",transfer);
        println!("Transfered Amount: {:?}", transfer);
        println!("Okoth New Balance: {}", accounts[0].balance);
        println!("Bob New Balance: {}", accounts[1].balance);
        println!("Okoth sent {} to Bob, increasing Bob's balance from {} to {}", transfer, pre_transfer_recipient_balance, accounts[1].balance);
    } else {
        println!("Insufficient Balance");
    }


    //Function to Delete Account at Index 0
    println!("//DELETE ACCOUNT AT INDEX 0");
    accounts.remove(0);
    println!("{:?}", accounts);
        
}



#[derive(Debug)]
struct Account {
    name: String,
    balance: f64,
}

impl Account {
    pub fn new(name: &str, balance: f64) -> Account {
        Account {
            name: name.to_string(),
            balance: balance,
        }
    }
}
