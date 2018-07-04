use std::cmp::Ord;


trait Maximum {
    type Item;
    fn maximum(&self) -> Option<&Self::Item>;
}

impl<A: Ord> Maximum for Vec<A> {
    type Item = A;
    fn maximum(&self) -> Option<&Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let mut item = self.first().unwrap();
        for i in self {
            if item < i {
                item = i;
            }
        }
        Some(item)
    }
}

            

fn main() {
    let v = vec![1,2,3,4,100,5,6,7,78,9];
    let max = v.maximum();
    let max_2 = v.into_iter().max();
    println!("{:?} is the maximum im Vec", max);
}
