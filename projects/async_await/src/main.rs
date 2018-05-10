

extern crate futures_await as futures;
use futures::prelude::*;
use futures::future::*;


fn create_future(x: bool) -> impl Future<Item = i32, Error = i32> {
    if x {
        ok::<i32,i32>(0)
    }else{
        err::<i32,i32>(1)
    }
}

fn main() {
    let t = create_future(false);
    println!("Hello, main");
    println!("Hello, {:?}", t.wait());
}
