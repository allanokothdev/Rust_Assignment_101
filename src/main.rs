mod account;

use account::Account;
use account::AccountOperations;
use account::transfer_money;

fn main() {

    //Initialize Vector Database to store Accounts
    let mut _accounts: Vec<Account> = Vec::new();

    //CREATE NEW ACCOUNT
    let account_one = Account::new("savings.allanokoth.testnet","Savings",1000.0);
    _accounts.push(account_one);
    println!("Account created: {:?}", _accounts[0]);

    //CREATE NEW ACCOUNT
    let account_two = Account::new("investment.allanokoth.testnet","Investments",1000.0);
    _accounts.push(account_two);
    println!("Account created: {:?}", _accounts[1]);

    //UPDATE OPERATION
    _accounts[0].update_account("Fixed Savings");
    println!("Account Updated: {:?}", _accounts[0]);

    //DEPOSIT OPERATION
    _accounts[0].deposit_money(10000.0);
    println!("Account Deposit: {:?}",_accounts[0]);

    //WITHDRAW OPERATION
    _accounts[0].withdraw_money(2000.0);
    println!("Account Withdrawal: {:?}", _accounts[0]);

    //MONEY TRANSFER OPERATION
    let amount = 1000.0;
    transfer_money(amount, &mut _accounts);
    println!("Money Transfer of {} from {:?} to {:?}",amount,_accounts[0],_accounts[1]);


}
