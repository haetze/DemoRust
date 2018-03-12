#![feature(proc_macro, conservative_impl_trait, generators)]



extern crate futures_await as futures;
extern crate futures_cpupool;


use futures::prelude::*;
use futures_cpupool::CpuPool;
use futures::sync::mpsc::*;
use std::thread;
        



#[async]
fn create_future<A: 'static, B>(x : A) -> Result<A, B> {
    Ok(x)
}


fn main() {
    let a : usize = 10;
    let pool = CpuPool::new(a);
    let mut future_vec = Vec::new();
    let mut channel_vec = Vec::new();
    for i in 0..a{
        let (mut tx, mut rx) = channel::<_>(1);
        let channel_future = rx.into_future();
        let result = channel_future.and_then(move |(r,z)| {
            println!("In Channel future: {} in Thread: {:?}", i, thread::current().id());
            let y = match r {
                Some(n) => Ok(n),
                _       => Err(((), z)),
            };
            y
        });

        let thread_handle = thread::spawn(move || {
            println!("Sending on Channel for future {} on threadID {:?}", i, thread::current().id());
            tx.send(a-i).wait().expect("Unable to send!");
        });
        
        channel_vec.push(thread_handle);
        let f = create_future(i).and_then(move |x| {
            println!("Future {} running in Thread: {:?}", i, thread::current().id());
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running in Thread: {:?}", i, thread::current().id());
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running in Thread: {:?}", i, thread::current().id());
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running in Thread: {:?}", i, thread::current().id());
            Ok(x)
        }).and_then(move |x| {
            println!("Future {} running in Thread: {:?}", i, thread::current().id());
            Ok(x)
        });

        let t = result.join(f).and_then(|(a,b)| Ok(a+b));
        future_vec.push(pool.spawn(t));
    }
    

    for handle in channel_vec {
        let _ = handle.join();
    }
        
    for v in future_vec {
        println!("Result of a Future: {:?}", v.wait());
    }

    
    println!("Main");

}
