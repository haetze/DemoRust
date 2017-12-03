struct Product<A, B> {
    x: A,
    y: B,
}

impl<A, B> Product<A,B> {
    fn map(&self, f:fn(&A) -> A, g:fn(&B) -> B) -> Product<A,B> {
        Product{x:f(&self.x), y:g(&self.y)}
    }
}


fn main() {
    let p = Product{x:12, y:12};
    println!("({},{})", p.x, p.y);
    let q = p.map(|x| 2*x, |y| y*y);
    println!("({},{})", q.x, q.y);
}
