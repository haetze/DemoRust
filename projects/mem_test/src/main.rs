#[derive(Debug)]
struct Container<A> {
    c: A,
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
}
