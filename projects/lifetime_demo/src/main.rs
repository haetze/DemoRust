
fn test<'a, A,B>(x: &'a A, y: &'a B, b: bool) -> Result<&'a A, &'a B> {
    if b {
        Ok(x)
    }else {
        Err(y)
    }
}


fn main() {
    let result = test(&1,&2, true);
    println!("{:?}", result);
    
    let result = test(&1,&2, false);
    println!("{:?}", result);
}
