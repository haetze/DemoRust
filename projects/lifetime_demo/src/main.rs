
// Works
fn test_1<'a, A,B>(x: &'a A, y: &'a B, b: bool) -> Result<&'a A, &'a B> {
    if b {
        Ok(x)
    }else {
        Err(y)
    }
}

// Doesn't work, needs lifetime explicitly, like above
// fn test_2<A,B>(x: &A, y: &B, b: bool) -> Result<&A, &B> {
//     if b {
//         Ok(x)
//     }else {
//         Err(y)
//     }
// }

//Also works with different lifetime
fn test_3<'a, 'b, A,B>(x: &'a A, y: &'b B, b: bool) -> Result<&'a A, &'b B> {
    if b {
        Ok(x)
    }else {
        Err(y)
    }
}


fn main() {
    let result = test_1(&1,&2, true);
    println!("{:?}", result);
    
    let result = test_1(&1,&2, false);
    println!("{:?}", result);


    let result = test_3(&1,&2, true);
    println!("{:?}", result);
    
    let result = test_3(&1,&2, false);
    println!("{:?}", result);
}
