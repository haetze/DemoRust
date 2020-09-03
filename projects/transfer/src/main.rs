use std::sync::Arc;
use std::sync::Mutex;

mod bank;

fn main() {
    use bank::Account;
    use bank::Cash;
    use crate::bank::transfer;
        
    let acc_1 = Arc::new(Mutex::new(Account::new(1000)));
    let acc_2 = Arc::new(Mutex::new(Account::new(0)));
    let cash = Arc::new(Mutex::new(Cash::new()));

    println!("{:?}", acc_1);
    println!("{:?}", acc_2);
    println!("\n");
    
    transfer(&acc_1, &acc_2, 200);
    
    println!("{:?}", acc_1);
    println!("{:?}", acc_2);
    println!("\n");

    println!("{:?}", acc_1);
    println!("\n");

    transfer(&cash, &acc_1, 20000);
    
    println!("{:?}", acc_1);
}
