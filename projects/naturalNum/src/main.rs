
enum Nat<'a>{
    Epsilon,
    Nat(&'a Nat<'a>)
}


fn to_int(nat: &Nat) -> i32{
    match nat{
        &Nat::Epsilon => 0,
        &Nat::Nat(prev) => 1+to_int(prev),
    }
}



fn main() {

    let x = Nat::Epsilon;
    let y = Nat::Nat(&x);
    println!("{}", to_int(&y));
}
