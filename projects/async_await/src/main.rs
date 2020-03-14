use std::time::Duration;
use async_std::task;
use async_std::future;
use async_std::prelude::*;

async fn test_1() -> Result<u32,()> {
    task::sleep(Duration::from_secs(5)).await;
    println!("A");
    Ok(0)
}

async fn test_2() -> Result<u32,()> {
    task::sleep(Duration::from_secs(10)).await;
    println!("B");
    Ok(0)
}

#[async_std::main]
async fn main() -> () {
    let b = test_2();
    let a = test_1();
    let c = a.join(b);
    let d = c.await;
    ()
}
