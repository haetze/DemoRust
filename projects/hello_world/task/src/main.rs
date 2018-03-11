#![feature(proc_macro, conservative_impl_trait, generators)]


extern crate futures_await as futures;
extern crate futures_cpupool;

use futures::prelude::*;
use futures::future::*;
use futures_cpupool::CpuPool;
use futures::sync::mpsc::*;
use std::thread;
        

#[async]
fn create_future(x :i32) -> Result<i32, String> {
    Ok(x)
}


fn main() {
    let pool = CpuPool::new(1);
    let a : i32 = 1;
    let mut future_vec = Vec::new();
    let mut future_vec2 = Vec::new();
    let mut channel_vec = Vec::new();
    for i in 0..a{
        let (mut tx, mut rx) = channel::<i32>(1);
        let channel_future = rx.into_future();
        let result = channel_future.and_then(|result| {
            let y : FutureResult<i32, i32> = match result {
                (Some(n), _) => ok(n),
                (_,_)        => err(0),
            };
            //let m :Result<i32, i32>= Err(3);
            //println!("{:?}", m);
            Ok(y)
        });
        channel_vec.push(tx);
        let f = create_future(i).and_then(move |x| {
            println!("Future {} running", i);
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running", i);
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running", i);
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running", i);
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running", i);
            Ok(x)
        });
        
        future_vec.push(pool.spawn(f));
        future_vec2.push(pool.spawn(result));
    }

    channel_vec.reverse();
    let mut b = a;
    thread::sleep(std::time::Duration::from_millis(100));
    for v in channel_vec {
        v.send(b).wait().expect("Unable to send!");
        b = b - 1;
    }
    
    /*for v in future_vec {
        println!("Result of a Future: {:?}", v.wait());
    }*/

    for v in future_vec2 {
        println!("Channel future result: {:?}", v.wait());
    }

    
    println!("Main");

}
