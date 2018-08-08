trait Functor<T, U> {
    type Out;
    fn fmap<F>(self, f: F) -> Self::Out
        where F: Fn(T) -> U;
    
}

trait Monad<T, U> : Functor<T, U> {
    type MonadOut;
    fn eta(t: T) -> Self;
    fn bind<F>(self, f: F) -> Self::MonadOut
        where F: Fn(T) -> Self::MonadOut;
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

impl<A, B> Monad<A, B> for Option<A> {
    type MonadOut = Option<B>;
    fn eta(a: A) -> Option<A> {
        Some(a)
    }
    fn bind<F>(self, f: F) -> Self::MonadOut
        where F: Fn(A) -> Self::MonadOut {
        match self {
            None => None,
            Some(a) => f(a),
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
