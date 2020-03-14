use std::time::Duration;
use async_std::task;
use async_std::prelude::*;
use std::time::SystemTime;

async fn test_1() -> Result<u32,()> {
    task::sleep(Duration::from_secs(1)).await;
    // loop {
    // }
    println!("A");
    Ok(0)
}

async fn test_2() -> Result<u32,()> {
    task::sleep(Duration::from_secs(1)).await;
    println!("B");
    Ok(0)
}

#[async_std::main]
async fn main() -> () {
    let before =  SystemTime::now();
    let b = test_2();
    let a = test_1();
    // let c = a.join(b);
    let c = task::spawn(a);
    let d = task::spawn(b);
    let _e = c.join(d).await;
    let after =  SystemTime::now();
    println!("{:?}", before);
    println!("{:?}", after);
    ()
}
