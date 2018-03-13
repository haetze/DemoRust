
enum Nat{
    Epsilon,
    Nat(Box<Nat>)
}


impl std::cmp::PartialEq for Nat{
    fn eq(&self, other: &Nat ) -> bool{
        match *self {
            Nat::Epsilon => match *other {
                Nat::Epsilon => true,
                _            => false
            },
            Nat::Nat(ref x) => match *other {
                Nat::Nat(ref y) => *x == *y,
                _           => false
            }
        }
    }

}

impl Nat {
    fn to_int(self) -> i32{
        match self {
            Nat::Epsilon => 0,
            Nat::Nat(x)  => 1+Nat::to_int(*x)
        }
    }

    fn new(x: u32)-> Nat{
        Nat::from_int(x)
    }
    
    fn from_int(x: u32) -> Nat{
        match x {
            0 => Nat::Epsilon,
            n => Nat::Nat(Box::new(Nat::from_int(n-1)))
        }
    }
}
            

fn main() {
    println!("{}", Nat::to_int(Nat::new(12)));
    println!("{}", Nat::new(5) == Nat::new(4));

}
