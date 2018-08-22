#![allow(dead_code)]
#![feature(box_patterns, box_notation)]

use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
struct Val<A> {
    val: A,
}


#[derive(Debug, Clone, PartialEq)]
struct Var<A> {
    var: String,
    phantom: PhantomData<A>,
}

#[derive(Debug, Clone, PartialEq)]
struct Lambda<A, B> {
    var : Var<A>,
    term: B,
}

#[derive(Debug, Clone, PartialEq)]
struct App<A, B> {
    fun : A,
    term: B,
}

#[derive(Debug, Clone, PartialEq)]
enum Term<A,B> {
    Val(Val<B>),
    Var(Var<B>),
    Lambda(Lambda<A,B>),
    App(App<A,B>),
}


fn main() {
    println!("Hello, world!");
}
