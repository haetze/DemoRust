#![feature(async_await, futures_api, await_macro)]

async fn test_1() -> Result<u32,()> {
    Ok(0)
}

async fn test_2() -> Result<u32,()> {
    let r = await!(test_1())?;
    Ok(r+2)
}

fn main() {
    // let r = test_2().poll();
    // println!("{:?}", r);
}
