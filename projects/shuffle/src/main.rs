extern crate rand;


/// A Trait that implements a random reordering of the
/// data present in the collection that implements this trait
trait Shuffle {
    fn shuffle(&mut self);
}

/// Shuffle for slice of any type
/// The Algorithm used is from R. A. Fisher and
/// F. Yates from D. Knuth's "The Art of Programming - Seminumerical
/// Algorithms Chapter 3.4.2 Algorithm P
/// A slight modification is made:
/// In step P3 k is set to (j*u).floor() +1.
/// The +1 is here omitted, because slices are indexed starting
/// at 0 and not like the sequence with 1.
/// Otherwise the original value at index 0 will
/// always end up at index.
/// # Demonstartion
/// ```
/// fn shuffle<A>(&mut [A]){
///
///       for j in (0..self.len()).rev() {
///           let u = rand::random::<f32>() % 1.0;
///           let k = (j as f32 * u).floor();
///           self.swap(j, k as usize);
///       }
/// }
///
/// let mut v: Vec<u32> = (0..10).collect();
/// shuffle(v);
/// assert_eq!(v[1], 0);
/// ```
impl<A> Shuffle for [A] {
    fn shuffle(&mut self){
        
        for j in (0..self.len()).rev() {
            let u = rand::random::<f32>() % 1.0;
            let k = (j as f32 * u).floor();
            self.swap(j, k as usize);
        }
    }
}

fn main() {
    let mut v: Vec<u32> = (0..100).collect();
    v.shuffle();
    println!("{:?}", v);
}
