use async_std::task;
use async_std::prelude::FutureExt;
use std::time::SystemTime;
use futures::future::{BoxFuture, FutureExt as FExt};

const LIMIT : usize = 256;

fn split(arr : &[u32]) -> (&[u32], &[u32]) {
    let l = arr.len()/2;
    return (&arr[..l], &arr[l..]);
}

fn merge(a : &[u32], b : &[u32]) -> Vec<u32> {
    let l = a.len();
    let m = b.len();
    let mut v = vec![0;l+m];

    let mut i = 0;
    let mut j = 0;
    for e in &mut v {
	if i >= l {
	    *e = b[j];
	    j += 1;
	} else if j >= m {
	    *e = a[i];
	    i += 1;
	} else if a[i] > b[j] {
	    *e = b[j];
	    j += 1;
	} else {
	    *e = a[i];
	    i += 1;
	}
    }

    return v;
}

fn sort(mut arr : Vec<u32>) -> BoxFuture<'static, Vec<u32>> {
    async move {
	if arr.len() <= LIMIT {
	    arr.sort();
	    return arr;
	}
	let (a,b) = split(&arr);
	let a = sort(a.to_vec());
	let b = sort(b.to_vec());
	let a = task::spawn(a);
	let b = task::spawn(b);
	let (a,b) = a.join(b).await;
	merge(&a,&b)
    }.boxed()
}

fn sort_seq(mut arr : Vec<u32>) -> Vec<u32> {
    if arr.len() <= LIMIT {
	arr.sort();
	return arr;
    }
    let (a, b) = split(&arr);
    let a = sort_seq(a.to_vec());
    let b = sort_seq(b.to_vec());
    let arr_s = merge(&a,&b);
    return arr_s;
}




#[async_std::main]
async fn main() -> () {
    let n : u32 = (2 as u32).pow(25);
    
    //Merge Sort (parallel)
    let v : Vec<u32> = (1..n).rev().collect();
    let start =  SystemTime::now();
    let _u_sorted = task::spawn(sort(v)).await;
    let merge_p_time = start.elapsed().unwrap().as_nanos();
    println!("{:?}", merge_p_time);

    //Merge Sort (seq)
    let v : Vec<u32> = (1..n).rev().collect();
    let start =  SystemTime::now();
    let _u_sorted = sort_seq(v);
    let merge_s_time = start.elapsed().unwrap().as_nanos();
    println!("{:?}", merge_s_time);

    //Merge Sort (seq)
    let mut v : Vec<u32> = (1..n).rev().collect();
    let start =  SystemTime::now();
    v.sort();
    let sort_b_time = start.elapsed().unwrap().as_nanos();
    println!("{:?}", sort_b_time);

    return 

}
