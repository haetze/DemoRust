#![allow(unused_imports, dead_code)]

extern crate rayon;

use rayon::prelude::*;

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
    let result = (1..number)
        .into_par_iter()
        .all(|i| is_prime(i) == is_prime_2(i));
    println!("{}", result);
    
    
    // println!("{:?} as Vec", vec);
    // println!("{:?} as HashSet", set);
    let vec: Vec<_> = par_primes_til(number);
    println!("Vec only primes? {}", vec.iter().all(|x| is_prime_2(*x)));
    let set: HashSet<_> = primes_til(number);
    println!("Set only primes? {}", set.iter().all(|x| is_prime_2(*x)));
    
    
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
        _     => (2..u).into_par_iter().all(|x| u % x != 0),
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

fn par_primes_til<A: FromIterator<u32>>(u: u32) -> A {
    let mut v = vec![2];
    for i in 3..u {
        if v.par_iter().all(|x| i % x != 0) {
            v.push(i);
        }
    }
    v.into_iter().collect()
}
