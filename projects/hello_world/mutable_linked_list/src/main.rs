
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
            List::Cons(ref h, ref t) => write!(f, "({} {}\n)", h, t)
        }        
    }
}

trait ImmutableMap<A,B> {
    fn immutable_map(&self, fn(&A) -> B) -> List<B>;
}

trait MutableMap<A>{
    fn mutable_map(&mut self, fn(&A) -> A);
}

fn list_create(i:i32) -> List<i32>{
    if i == 0{
        List::Nil
    }else{
        List::Cons(Box::new(i), Box::new(list_create(i-1)))
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
            

fn main() {
    let mut l = list_create(10);
    l.mutable_map(|x| 2*x);
    let l2 = l.immutable_map(|x| list_create(*x));
    //println!("{}", l);
    println!("{}", l2);
}
