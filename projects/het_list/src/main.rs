use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug)]
enum EmptyList{
    Nil,
}

#[derive(Debug)]
enum NoneEmptyList<A, B> {
    Cons(A, B),
}

fn head<'a, A:'a, B:'a> (x: &'a NoneEmptyList<A, B>) -> &'a A{
    use NoneEmptyList::*;
    match *x {
        Cons(ref h, _) => h,
    }
}

fn tail<'a, A:'a, B:'a> (x: &'a NoneEmptyList<A, B>) -> &'a B{
    use NoneEmptyList::*;
    match *x {
        Cons(_, ref t) => t,
    }
}

trait Print {
    fn print(&self);
}


impl Print for EmptyList {
    fn print(&self){
        print!("[]");
    }
}


impl<A: Debug, B: Print> Print for NoneEmptyList<A, B> {
    fn print(&self){
        match *self {
            NoneEmptyList::Cons(ref h, ref t) => {
                print!("{:?} : ", h);
                t.print();
            }
        }
    }
}   

trait Sumable<Result: Add>{
    fn sum(self) -> Result;
}


impl Sumable<i32> for EmptyList {
    fn sum(self) -> i32 {
        0
    }
}

impl Sumable<f32> for EmptyList {
    fn sum(self) -> f32 {
        0.0
    }
}


impl<A: Add<A>, B: Sumable<A>> Sumable<A::Output> for NoneEmptyList<A, B>
    where A::Output: std::ops::Add
{
    fn sum(self) -> A::Output {
        match self {
            NoneEmptyList::Cons(h, t) => h + t.sum(),
        }
    }
}


fn main() {
    use NoneEmptyList::*;
    use EmptyList::*;
    let l = Nil;
    let l : NoneEmptyList<f32, _> = Cons(10.0, l);
    let l : NoneEmptyList<f32, _> = Cons(20.0, l);
    // let l = Cons("0", l);
    // println!("{:?}", l);
    // let head = head(&l);
    // println!("{:?}", head);
    // let tail = tail(&l);
    // println!("{:?}", tail);
    l.print();
    println!("");
    let sum = l.sum();
    println!("{}", sum);
}
