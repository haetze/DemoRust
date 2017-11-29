#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;
use std::mem;

enum List<A> {
    Nil,
    Cons(Box<A>, Box<List<A>>),
}

impl<A: fmt::Display> fmt::Display for List<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            List::Nil => write!(f, "Nil"),
            List::Cons(ref h, ref t) => write!(f, "({} {})", h, t)
        }        
    }
}

trait ImmutableMap<A,B> {
    fn immutable_map(&self, fn(&A) -> B) -> List<B>;
}

trait MutableMap<A>{
    fn mutable_map(&mut self, fn(&A) -> A);
}

fn list_create(i:f32) -> List<f32>{
    if i <= 0.0{
        List::Nil
    }else{
        List::Cons(Box::new(i), Box::new(list_create(i-1.0)))
    }
}

impl<A> MutableMap<A> for List<A>{
    fn mutable_map(&mut self, f:fn(&A) -> A) {
        match *self {
            List::Nil => (),
            List::Cons(ref mut h, ref mut t) => {
                let a = f(&h);
                mem::replace(&mut *h, Box::new(a));
                t.mutable_map(f);
            },
        }
    }
}

impl<A,B> ImmutableMap<A,B> for List<A>{
    fn immutable_map(&self, f: fn(&A) -> B) -> List<B>{
        match *self{
            List::Nil => List::Nil,
            List::Cons(ref h, ref t) => List::Cons(Box::new(f(&h)), Box::new(t.immutable_map(f))),
        }
    }
}

impl<A> List<A>{
    fn new() -> List<A>{
        List::Nil
    }

    fn add(self, a:A) -> List<A>{
        List::Cons(Box::new(a), Box::new(self))
    }
    fn remove_head(self) -> List<A>{
        match self {
            List::Nil => panic!("Empty List"),
            List::Cons(h, t) => *t,
        }
    }
    
}
            

fn main() {
    let l1 : List<f32> = List::new();
    let l2 = l1.add(2.0);
    let mut l3 = l2.add(4.0);
    l3.mutable_map(|x| x.sqrt());
    println!("{}", l3);
    //let empty : List<i32> = List::new();
    //empty.remove_head();
    let l4 = l3.remove_head();
    println!("{}", l4);
    //println!("{}", l2);
    /*let mut a = 2;
    loop{
        a = a + a;

        println!("{}",a);
            

    }*/
}
