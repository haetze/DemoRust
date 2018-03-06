#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
extern crate futures_cpupool;



use futures::prelude::*;
use futures_cpupool::CpuPool;
use std::{thread, time};

#[async]
fn create_future(x :i32) -> Result<i32, i32> {
    Ok(x)
}

const A : u64 =  1;

fn main() {
    let future_1 = create_future(1234);
    let future_2 = create_future(5678);
    let f_1 = future_1.map(|x| {
        println!("Future 1, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 1, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 1, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 1, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 1, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    });
    let f_2 = future_2.map(|x| {
        println!("Future 2, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 2, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 2, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 2, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    }).map(|x| {
        println!("Future 2, x = {}", x);
        let millis = time::Duration::from_millis(A);
        thread::sleep(millis);
        x + 1
    });
    let pool = CpuPool::new(8);
    let r_1 = pool.spawn(f_1);
    let r_2 = pool.spawn(f_2);
   
    println!("Main");
    println!("({:?}, {:?})", r_1.wait(), r_2.wait());
}
