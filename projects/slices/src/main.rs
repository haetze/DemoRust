#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt;

pub trait OuterMap<A, B> {
    type Output;
    fn outer_map(&self, f: fn(&A)-> B) -> Self::Output;
    
}

pub trait Functor<A, B> {
    type Output;
    fn fmap(&self, f: fn(&A) -> B) -> Self::Output;
}

    
enum List<A> {
    Nil,
    Cons(A, Box<List<A>>),
}


pub trait InnerMap<A>{
    fn inner_map(&mut self, fn(&A)-> A);
}

impl<A: fmt::Display> fmt::Display for List<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            List::Nil => write!(f, "Nil"),
            List::Cons(ref h, ref t) => write!(f, "({} {})", h, t)
        }
            
        
    }
}

impl<A,B> OuterMap<A,B> for [A]{
    type Output = Vec<B>;
    fn outer_map(&self, f:fn(&A) -> B) -> Vec<B>{
        self.iter().map(f).collect()
    }

}

impl<A> InnerMap<A> for [A] {
    fn inner_map(&mut self, f:fn(&A) -> A){
        for x in 0..self.len(){
            self[x] = f(&self[x]);
        }
    }
}

impl<A, B> Functor<A,B> for [A] {
    type Output = Vec<B>;
    fn fmap(&self, f:fn(&A) -> B) -> Vec<B>{
        self.iter().map(f).collect()
    }
}


impl<A,B> Functor<A,B> for List<A> {
    type Output = List<B>;
    fn fmap(&self, f:fn(&A) -> B) -> List<B>{
        match *self{
            List::Nil => List::Nil,
            List::Cons(ref h, ref t) => {List::Cons(f(&h), Box::new(t.fmap(f)))
        }
    }
}
            

fn sum(slice: &[i32]) -> i32{
    let mut sum = 0;
    for &i in slice{
        sum += i;
    }
    return sum;
}



fn double(slice: &mut [i32]){
    for x in 0..slice.len(){
        slice[x] = slice[x]*2
    }
}

fn double_2(slice: &[i32]) -> Vec<i32>{
    slice.iter().map(|a| 2*a).collect()
}


fn print_on_line<T: std::fmt::Display>(slice: &[T]){
    for x in slice {
        println!("{}", x)
    }
}


fn mut_map<A>(f: fn(&A) -> A, slice: &mut [A]){
    for x in 0..slice.len(){
        slice[x] = f(&slice[x]);
    }
}

fn create_list_from(i: f32) -> List<f32>{
    if i == 0.0{
        return List::Nil
    }else{
        List::Cons(i + 0.1, Box::new(create_list_from(i-1.0)))
    }
}

impl<A: Copy> List<A>{
    fn new(a:A, i:i32)-> List<A>{
        if i == 0{
            List::Nil
        }else{
            List::Cons(a, Box::new(List::new(a, i-1)))
        }
    }
}

                


fn main() {
    let mut slice : Vec<i64>= (1..101).collect();
    let mut s = ["Hi".to_string(), "Hi".to_string()];
    s.inner_map(|x| format!("{}{}",x,x));

    //double(&mut slice);
    //let s = slice.map(|a| 2*a);
    slice.inner_map(|a| a*a);
    //let s_2 = double_2(&s);

    
    //mut_map(|a| 2*a, &mut slice);
    //mut_map(|a| {println!("{}", a);
    //             *a}, &mut slice);
    let l1 = create_list_from(100.0);
    let l4 = l1.fmap(|x| x*x);
    //let l5 = List::new("hi\n", 100);
    //let l6 = l5.fmap(|x| x.to_string());
    //panic!("Help");
    //der inlined das
    //also ist das nicht IO safe
    /*let n = (0..10)
        .map(|x| {
            print!("from map\n");
            2*x})
        .filter(|&x| {
            print!("from filter\n");
            x < 5
        })
        .fold(0,|acc, x| acc+x);
    print!("{}\n", n);*/
            
             
                
    //println!("{}", l4);
    //println!("{}", l5);
    //println!("{:?}", s);
    //println!("{:?}", slice);
        
        
}
