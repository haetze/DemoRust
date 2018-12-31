fn main() {
    let n = 43;
    let v: Vec<i32> = (0..100).collect();
    let b = v.b_search(n);
    println!("{} in Vec?: {}", n, b);
}

// Binary Search, search should run in O(log n), n = size of the
// DS
trait BSearch<A, I>{
    fn b_search(&self, x: A) -> bool;
    fn b_search_i(&self, x: A, min: I, max:I) -> bool;
}

impl<A: std::cmp::PartialEq+std::cmp::PartialOrd> BSearch<A, usize> for Vec<A> {
    fn b_search(&self, x: A) -> bool {
        self.b_search_i(x, 0, self.len())
    }
    fn b_search_i(&self, x: A, min: usize, max: usize) -> bool {
        if max == min {
            self[max] == x
        }else {
            let i = ((max - min)/2) + min; 
            if self[i] == x {
                true
            } else if self[i] < x {
                self.b_search_i(x, i, max)
            } else {
                self.b_search_i(x, min, i)
            }
        }
    }
}
