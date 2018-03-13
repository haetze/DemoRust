#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;

use futures::prelude::*;

#[async]
fn c_f(x:i32) -> Result<i32, i32> {
    Ok(x)
}


fn main() {
    let future   = c_f(1234);
    let future_2 = future.map(|x| {
        println!("Hello, Future!");
        x+10});
    println!("Hello, World!", );
    println!("Result: {:?}", future_2.wait());
                              

}
