extern crate rand;



trait Shuffle {
    fn shuffle(&mut self);
}

impl<A> Shuffle for [A] {
    fn shuffle(&mut self){
        
        for j in (0..self.len()).rev() {
            let u = rand::random::<f32>() % 1.0;
            let k = (j as f32 * u).floor() + 1.0;
            self.swap(j, k as usize);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
