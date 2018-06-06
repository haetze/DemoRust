#[derive(Debug)]
enum List<A, B> {
    Nil,
    Cons(A, B),
}


fn main() {
    use List::*;
    let l: List<(),()> = Nil;
    let l = Cons(0, l);
    let l = Cons(0, l);
    let l = Cons("0", l);
    println!("{:?}", l);
}
