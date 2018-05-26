use std::iter::FromIterator;

#[derive(Debug)]
struct Container<A> {
    c: A,
}



impl<A> Container<A> {
    fn change(&mut self, x:A){
        self.c = x;
    }

    fn use_fn(&mut self, f: fn(&A) -> A){
        self.c = f(&self.c);
    }
}

#[derive(Debug)]
enum InnerList<A> {
    Nil,
    Cons(A, Box<InnerList<A>>),
}

#[derive(Debug)]
struct List<A> {
    l: Box<InnerList<A>>,
}

impl<A> List<A> {
    fn insert(&mut self, a:A){
        use std::mem::swap;
        let mut l = Box::new(InnerList::Nil);
        swap(&mut l, &mut self.l);
        self.l = Box::new(InnerList::Cons(a, l));
    }
}

struct ListIter<'a, A: 'a> {
    data: Vec<&'a A>,
}

impl<'a, A: 'a> ListIter<'a, A> {
    fn put_in_vec(&mut self, l: &'a List<A>) {
        let mut temp_vec = Vec::new();
        let mut inner_list = &l.l;
        while let InnerList::Cons(ref head, ref tail) = **inner_list {
            temp_vec.push(head);
            inner_list = tail;
        }
        while let Some(element) = temp_vec.pop() {
            self.data.push(element);
        }
    }
}

impl<'a, A: 'a> Iterator for ListIter<'a, A>{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item>{
        self.data.pop()
    }
}

impl<'a, A: 'a> IntoIterator for &'a List<A>{
    type Item = &'a A;
    type IntoIter = ListIter<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = ListIter{ data: Vec::new()};
        iter.put_in_vec(&self);
        iter
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<I: IntoIterator<Item = A>> (iter: I) -> Self {
        let mut l = InnerList::Nil;
        let mut temp_vec = Vec::new();
        for i in iter {
            temp_vec.push(i);
        }
        while let Some(i) = temp_vec.pop() {
            l = InnerList::Cons(i, Box::new(l));
        }
        List{ l: Box::new(l),}
    }
}

fn pow(base: i32, exp: i32) -> i32 {
    let mut result = base;
    for _ in 1..exp {
        result = result * base;
    }
    result
}


fn main() {
    let mut c = Container{c: 32};
    println!("{:?}", c);
    c.change(21);
    println!("{:?}", c);
    c.use_fn(|x| x*2);
    println!("{:?}", c);
    let mut l: List<i32> = List{ l: Box::new(InnerList::Nil)};
    println!("{:?}", l);
    for i in 0..10 {
        l.insert(i);
        // println!("{:?}", l);
    }
    let l: List<i32> = l.into_iter().map(|x| pow(*x, *x)).collect();
    println!("{:?}", l);

}
