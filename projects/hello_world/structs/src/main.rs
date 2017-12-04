struct Product<A, B> {
    x: A,
    y: B,
}

struct Product2<'a, A:'a, B:'a>{
    x: &'a mut A,
    y: &'a mut B,
}

impl<A, B> Product<A,B> {
    fn map(&self, f:fn(&A) -> A, g:fn(&B) -> B) -> Product<A,B> {
        Product{x:f(&self.x), y:g(&self.y)}
    }
}

impl<A,B> Product<A,B> {
    fn set_x(&mut self, a: A){
        self.x = a
    }
    fn set_y(&mut self, a: B){
        self.y = a
    }
    fn map_x(&mut self, f:fn(&A) -> A){
        let x = f(&self.x);
        self.set_x(x);
    }
    fn map_y(&mut self, f:fn(&B) -> B){
        let y = f(&self.y);
        self.set_y(y);
    }
    fn mut_map(&mut self, f:fn(&A) -> A, g:fn(&B) -> B){
        self.map_x(f);
        self.map_y(g);
    }
}

fn main() {
    let mut p1 = Product{x:12, y:12};
    //let q1 = Product2{x:&mut 12, y:&mut 34};
    println!("Original: ({},{})", p1.x, p1.y);
    p1.set_x(133);
    println!("Erstes Paramter neu gesetzt:({},{})", p1.x, p1.y);
    p1.map_x(|x| x*2);
    println!("Erstes Parameter gemapped: ({},{})", p1.x, p1.y);
    p1.map_y(|y| y+2);
    println!("Zweites Parameter gemapped: ({},{})", p1.x, p1.y);
    //println!("({},{})", p1.x, p1.y);
    p1.mut_map(|x| x*x, |y| 3*y);
    println!("Beide Parameter gemapped: ({},{})", p1.x, p1.y);
    //let p2 = p1.map(|x| 2*x, |y| y*y);
    //println!("({},{})", p2.x, p2.y);
}
