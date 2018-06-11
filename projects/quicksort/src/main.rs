extern crate rand;

use std::cmp::PartialOrd;
use std::mem::swap;
use rand::Rng;

trait Split: Sized{
    type Inner;
    fn split(self, p: impl Fn(&Self::Inner) -> bool) -> (Self, Self);
    fn split_default(self) -> (Self, Self);
}

trait QuickSort: Split{
    fn quicksort(&mut self);
}

impl<A: PartialOrd + std::fmt::Debug> QuickSort for Vec<A> {
    fn quicksort(&mut self){

        if self.len() <= 1 {
            return;
        }
        let mut vec = Vec::new();
        swap(self, &mut vec);
        let (mut v, mut w) = vec.split_default();
        v.quicksort();
        w.quicksort();
        for i in v {
            self.push(i);
        }
        for i in w {
            self.push(i);
        }
    }
}

impl<A: PartialOrd> Split for Vec<A> {
    type Inner = A;
    fn split(self, p: impl Fn(&Self::Inner) -> bool) -> (Self, Self){
        let mut satisfies_p = Vec::new();
        let mut not_satisfies_p = Vec::new();
        for i in self{
            if p(&i) {
                satisfies_p.push(i);
            }else{
                not_satisfies_p.push(i);
            }
        }       
        (satisfies_p, not_satisfies_p)
    }

    fn split_default(mut self) -> (Self, Self){
        if self.len() == 0 {
            (Vec::new(), Vec::new())
        }else{
            let last = self.pop().unwrap();
            let (mut v, mut w) = self.split(|x| x.lt(&last));
            if v.len() < w.len() {
                v.push(last)
            }else {
                w.push(last);
            }
            (v, w)
                
        }
    }
}

#[test]
fn test_quicksort(){

    use std::thread;
    let mut joins = Vec::new();
    for _ in 0..8 {

        joins.push(thread::spawn( move ||
                                  
                                  for _ in 0..1000 {
                                      let mut rng = rand::thread_rng();
                                      let mut numbers_1 : Vec<i32> = Vec::new();
                                      let mut numbers_2 : Vec<i32> = Vec::new();
                                      for _ in 0..1000 {
                                          let num  = rng.gen_range(1, 21);
                                          numbers_1.push(num);
                                          numbers_2.push(num);
                                      }
                                      numbers_1.quicksort();
                                      numbers_2.sort();
                                      
                                      assert_eq!(numbers_1, numbers_2);
                                  }));
    }
    for h in joins {
        h.join().unwrap();
    }
}


fn main() {

    let mut rng = rand::thread_rng();
    let mut numbers_1 : Vec<u32> = Vec::new();
    for _ in 0..5 {
        let num  = rng.gen_range(1, 21);
        numbers_1.push(num);
    }
    numbers_1.quicksort();
    println!("{:?}", numbers_1);
    
    
}
