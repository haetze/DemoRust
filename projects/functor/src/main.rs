trait Functor<T, U> {
    type Out;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(T) -> U;
    
}

impl<A, B> Functor<A,B> for Option<A>{
    type Out = Option<B>;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(A) -> B {
        match self {
            None => None,
            Some(a) => Some(f(a)),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
