
fn add_one<'a>(slice : &[&'a i32]) -> Vec<&'a i32> {
    let mut v = Vec::new();
    for s in slice.iter() {
        v.push(*s);
    }
    v.push(&1);
    return v;
}


fn main() {
    let v : Vec<&i32> = vec![&1,&2,&3,&4];
    println!("{:?}", v);

    let v = add_one(v.as_slice());
    println!("{:?}", v);

    let ar : [&i32;4] = [&1,&2,&3,&4];
    println!("{:?}", ar);

    let ar = add_one(&ar);
    println!("{:?}", ar);
}
