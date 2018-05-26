#[derive(Debug)]
struct Container<A> {
    c: A,
}

#[derive(Debug)]
struct List<A> {
    l: Box<InnerList<A>>,
}

#[derive(Debug)]
enum InnerList<A> {
    Nil,
    Cons(A, Box<InnerList<A>>),
}

impl<A> List<A> {
    fn insert(&mut self, a:A){
        use std::mem::swap;
        let mut l = Box::new(InnerList::Nil);
        swap(&mut l, &mut self.l);
        self.l = Box::new(InnerList::Cons(a, l));
    }
}

impl<A> Container<A> {
    fn change(&mut self, x:A){
        self.c = x;
    }

    fn use_fn(&mut self, f: fn(&A) -> A){
        self.c = f(&self.c);
    }
}

fn main() {
    let mut c = Container{c: 32};
    println!("{:?}", c);
    c.change(21);
    println!("{:?}", c);
    c.use_fn(|x| x*2);
    println!("{:?}", c);
    let mut l: List<i32> = List{ l: Box::new(InnerList::Nil)};
    println!("{:?}", l);
    for i in 0..10 {
        l.insert(i);
        println!("{:?}", l);
    }
}
