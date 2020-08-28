

/*
  EXAMPLE OUTPUT:
    0x7ffeeefe224c
    Element 1 at location 0: 0x7f7fdec05b60
    Element 2 at location 1: 0x7f7fdec05b64
    Element 3 at location 2: 0x7f7fdec05b68
    Element 1 at location 0: 0x7f7fdec05b60
    Element 3 at location 1: 0x7f7fdec05b64

  Hence, remove copies the elements left to it.

*/

fn main() {
    let x : u32 = 0;
    println!("{:p}", &x);
    
    let mut v = vec![1,2,3];
    for (i, e) in v.iter().enumerate() {
	println!("Element {} at location {}: {:p}", e, i, e);
    }
    v.remove(1);
    for (i, e) in v.iter().enumerate() {
	println!("Element {} at location {}: {:p}", e, i, e);
    }

    
}
