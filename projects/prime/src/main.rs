use std::env;
use std::iter::FromIterator;
use std::collections::HashSet;

const DEFAULT_NUMBER: u32 = 1000;

fn main() {
    let number = match env::args().skip(1).next() {
        None => DEFAULT_NUMBER,
        Some(n) => match n.parse() {
            Ok(n) => n,
            Err(_) => DEFAULT_NUMBER,
        },
    };
    for i in 1..number {
        println!("{} is prime? {}", i, is_prime(i));
        println!("{} is prime? {}", i, is_prime_2(i));
    }
    let vec: Vec<_> = primes_til(number);
    let set: HashSet<_> = primes_til(number);
    println!("{:?} as Vec", vec);
    println!("{:?} as HashSet", set);
    
    
}

fn is_prime(u: u32) -> bool {
    if u == 0 || u == 1 {
        return false;
    } else {
        for i in 2..u {
            if u % i == 0 {
                return false;
            }
        }
    }
    return true;
}

fn is_prime_2(u: u32) -> bool {
    match u {
        0...1 => false,
        _     => (2..u).all(|x| u % x != 0),
    }
}

fn primes_til<A: FromIterator<u32>>(u: u32) -> A {
    let mut v = vec![2];
    for i in 3..u {
        if v.iter().all(|x| i % x != 0) {
            v.push(i);
        }
    }
    v.into_iter().collect()
}
