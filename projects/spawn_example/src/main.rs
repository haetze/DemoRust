#![allow(unused_must_use)]


use std::thread;
use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let n :i32 = match buffer.trim().parse() {
        Ok(i)   => i,
        Err(..) => {
            println!("No Number was typed in, use default 10");
            10
        },
    };
    
    let mut handler_vec = Vec::new();
    for i in 0..n{
        handler_vec.push(thread::spawn(move || {
            println!("From child:{}", i);
            i*i
        }));
    }

    for child in handler_vec{
        let i = child.join().unwrap();
        println!("From main:{}", i);
    }
                         
}
