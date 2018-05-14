#[derive(Debug)]
struct List<A> {
    list: LList<A>,
}

#[derive(Debug)]
enum LList<A> {
    Nil,
    Cons(A, Box<LList<A>>),
}

impl<A> LList<A> {
    fn length(&self) -> u32 {
        match *self{
            LList::Nil => 0,
            LList::Cons(_, ref tail) => 1 + tail.length(),
        }
    }

    fn map<B>(&self, f: fn(&A) -> B) -> LList<B> {
        match *self {
            LList::Nil => LList::Nil,
            LList::Cons(ref h, ref t) => LList::Cons(f(&h), Box::new(t.map(f))),
        }
    }
}

impl<A> List<A> {
    fn new() -> List<A> {
        List{
            list: LList::Nil,
        }
    }

    fn map<B>(&self, f: fn(&A) -> B) -> List<B> {
        List {
            list: self.list.map(f),
        }
    }
    
    fn add(mut self, a: A) -> List<A>{
        self.list = LList::Cons(a, Box::new(self.list));
        self
    }

    fn up_to(x: i32) -> List<i32>{
        let mut l = List::new();
        for i in 0..x {
            l = l.add(i);
        }
        l
    }

    fn length(&self) -> u32{
        self.list.length()
    }
        
}



fn main() {
    let list : List<i32>  = List::<i32>::up_to(100);
    //println!("{:?}", list);
    println!("{:?}", list.length());
    let list = list.map(|x| x * x);
    println!("{:?}", list);
}
