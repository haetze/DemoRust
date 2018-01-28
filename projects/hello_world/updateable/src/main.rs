use std::mem;
use std::fmt;

trait Updateable<K:PartialEq, V>{
    fn update(&mut self, k: K, v: V);
}

enum List<A> {
    Nil,
    Cons(Box<A>, Box<List<A>>),
}

fn list_create(i:f32) -> List<f32>{
    if i <= 0.0{
        List::Nil
    }else{
        List::Cons(Box::new(i), Box::new(list_create(i-1.0)))
    }
}

impl<A: fmt::Display> fmt::Display for List<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            List::Nil => write!(f, "Nil"),
            List::Cons(ref h, ref t) => write!(f, "({} {})", h, t)
        }        
    }
}

impl<V: PartialEq> Updateable<V, V> for List<V>{
    fn update(&mut self, k: V, v: V){
        match *self{
            List::Nil => (),
            List::Cons(ref mut h, ref mut t) => {
                if k == **h {
                    mem::replace(&mut *h, Box::new(v));
                    
                }else{
                    t.update(k, v);
                }
            }
        }
    }
}

impl<V> Updateable<usize, V> for Vec<V> {
    fn update(&mut self, k: usize, v: V){
        self[k] = v;
    }
}



fn main() {
    let mut v = vec![1,2,3,4,5,6];
    println!("{:?}", v);
    v.update(3, 12);
    println!("{:?}", v);
    let mut list = list_create(10.0);
    println!("{:}", list);
    list.update(5.0, 100.0);
    println!("{:}", list);
}
