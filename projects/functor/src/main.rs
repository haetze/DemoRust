use std::iter::IntoIterator;
use std::iter::FromIterator;

trait Functor<T, U> {
    type Out;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(T) -> U;
    
}


impl<A, B, I: IntoIterator<Item = A>> Functor<A, B> for I {
    type Out = Vec<B>;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(A) -> B {
        let mut v = Vec::new();
        for i in self {
            v.push(f(i));
        }
        v

    }
}

fn main() {
    let before = Some(12);
    println!("{:?}", before);
    let after = before.fmap(|x| x + 1);
    println!("{:?}", after);
}
