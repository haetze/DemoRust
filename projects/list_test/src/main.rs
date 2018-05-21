use std::ops::Index;
use std::iter::Iterator;
use std::iter::FromIterator;

#[derive(Debug)]
enum List<A> {
    Nil,
    Cons(A, Box<List<A>>),
}

impl<A> List<A> {
    fn new() -> List<A> {
        List::Nil
    }
}

impl<'a, A: 'a> ListIter<'a, A> {
    fn put_in_vec(&mut self, mut l: &'a List<A>) {
        let mut temp_vec = Vec::new();
        while let List::Cons(ref h, ref t) = *l {
            temp_vec.push(h);
            l = t;
        }
        while let Some(i) = temp_vec.pop() {
            self.l.push(i);
        }
    }
}
        

struct ListIter<'a, A: 'a> {
    l: Vec<&'a A>,
}

impl<'a, A: 'a> Iterator for ListIter<'a, A>{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item>{
        self.l.pop()
    }
}

impl<'a, A: 'a> IntoIterator for &'a List<A>{
    type Item = &'a A;
    type IntoIter = ListIter<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = ListIter{ l: Vec::new()};
        iter.put_in_vec(&self);
        iter
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<I: IntoIterator<Item = A>> (iter: I) -> Self {
        let mut l = List::Nil;
        let mut temp_vec = Vec::new();
        for i in iter {
            temp_vec.push(i);
        }
        while let Some(i) = temp_vec.pop() {
            l = List::Cons(i, Box::new(l));
        }
        l
    }
}
    
    

impl<A> Index<usize> for List<A> {
    type Output = A;

    fn index(&self, i: usize) -> &A {
        if i == 0 {
            match self {
                List::Cons(h,_) => h,
                _               => panic!("Out of bound"),
            }
        }else{
            match self {
                List::Cons(_,t) => t.index(i-1),
                _               => panic!("Out of bound"),
            }
        }
    }

}

            


fn main() {
    let mut list : List<i32>  = List::<i32>::new();
    for i in 0..10 {
        list = List::Cons(i, Box::new(list));
    }
    println!("{:?}", list);
    for i in &list {
        println!("{}", i);
    }
    // let v: List<_> = list.into_iter()
    //     .map(|x| x*2)
    //     .map(|x| format!("{}", x))
    //     .collect();
    // println!("{:?}", v);
}
