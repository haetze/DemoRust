extern crate rand;



trait Shuffle {
    fn shuffle(&mut self);
}

impl<A> Shuffle for [A] {
    fn shuffle(&mut self){
        use std::mem::swap;
        let t: f64 = self.len().into();
        for j in (0..self.len()).rev() {
            let U = rand::random::<f64>() % 1.0;
            let k = (j.into() * U).floor() + 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
