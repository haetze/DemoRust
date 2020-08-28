use std::time::SystemTime;


const SIZE : usize = 128; 

const fn bubble(mut arr : [usize; SIZE]) -> [usize; SIZE] {
    let mut n = 0;
    loop {
	if n == SIZE {
	    break;
	}
	let mut i = 1;
	loop {
	    if i == SIZE {
		break;
	    }
	    if arr[i] < arr[i-1] {
		let a = arr[i];
		arr[i] = arr[i-1];
		arr[i-1] = a;
	    }

	    i += 1;
	}
	
	
	n += 1;
    }
    arr
}

const fn create() -> [usize; SIZE] {
    let mut arr = [0; SIZE];
    let mut i = 0;
    loop {
	if i == SIZE {
	    break;
	}
	arr[i] = SIZE - i;

	i += 1;
    }

    arr
}


/*
  USES:
    nightly-x86_64-apple-darwin (default)
    rustc 1.48.0-nightly (397b390cc 2020-08-27)

  EXAMPLE OUTPUTS:
    Const time: 0
    Dynamic time: 37000

    Const time: 0
    Dynamic time: 38000
*/

fn main() {
    const ARR : [usize; SIZE] = create();
    let start =  SystemTime::now();
    const ARR_S : [usize; SIZE] = bubble(ARR);
    let time = start.elapsed().unwrap().as_nanos();
    println!("Const time: {:?}", time);

    let arr = create();
    let start =  SystemTime::now();
    let _arr = bubble(arr);
    let time = start.elapsed().unwrap().as_nanos();
    println!("Dynamic time: {:?}", time);
    
}
