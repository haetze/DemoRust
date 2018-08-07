
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

impl<A, B> Functor<A, B> for Vec<A> {
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

impl<A, B, E> Functor<A, B> for Result<A, E> {
    type Out = Result<B, E>;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(A) -> B {
        match self {
            Ok(a) => Ok(f(a)),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let before = Some(12);
    println!("{:?}", before);
    let after = before.fmap(|x| x + 1);
    println!("{:?}", after);
}
