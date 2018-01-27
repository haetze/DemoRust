#![allow(unused_must_use)]


use std::thread;
use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let n = match buffer.trim().parse::<i32>() {
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
            i*2
        }));
    }

    for child in handler_vec{
        let i = child.join().unwrap();
        println!("From main:{}", i);
    }
                         
}
