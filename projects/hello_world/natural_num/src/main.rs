enum Nat{
    Epsilon,
    Nat(Box<Nat>)
}


impl Nat {
    fn to_int(x: Nat) -> i32{
        match x {
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

}
