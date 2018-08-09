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

impl<A, B> Monad<A, B> for Vec<A> {
    type MonadOut = Vec<B>;
    fn eta(a: A) -> Vec<A> {
        vec![a]
    }
    fn bind<F>(self, f: F) -> Self::MonadOut
        where F: Fn(A) -> Self::MonadOut {
        let mut v = Vec::new();
        for e in self {
            v.push(f(e));
        }
        v.into_iter().flatten().collect()
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

impl<A, B, E> Monad<A, B> for Result<A, E> {
    type MonadOut = Result<B, E>;
    fn eta(a: A) -> Result<A, E> {
        Ok(a)
    }
    fn bind<F>(self, f: F) -> Self::MonadOut
        where F: Fn(A) -> Self::MonadOut {
        match self {
            Err(e) => Err(e),
            Ok(a) => f(a),
        }
    }
}


// This is remecent of the map and and_then
// function implemented on many types and
// used for async programming with
// futures in rust
fn main() {
    let before = Some(12);
    println!("{:?}", before);
    let after = before.fmap(|x| x + 1);
    println!("{:?}", after);
    let even_later = after.bind(|_| Some(0));
    println!("{:?}", even_later);
    let in_one_go = Some(12)
        .bind(|y| Some(y+1))
        .bind(|x| Some(x*2));
    println!("{:?}", in_one_go);
    
    
}
