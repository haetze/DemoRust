#![allow(dead_code)]

use std::marker::PhantomData;

type VarHold = char;

struct Var<A> {
    var: VarHold,
    ph : PhantomData<A>,
}

struct Lambda<A, B> {
    var  : Var<A>,
    term : Box<Term<(), B>>,
    ph_a : PhantomData<A>,
    ph_b : PhantomData<B>,
}

struct App<A, B> {
    lambda: Lambda<A, B>,
    term  : Box<Term<(), A>>,
    ph_a : PhantomData<A>,
    ph_b : PhantomData<B>,
}


enum Term<A,B> {
    V(Var<A>),
    L(Lambda<A, B>),
    A(App<A, B>),
}


fn main() {
    println!("Hello, world!");
}
