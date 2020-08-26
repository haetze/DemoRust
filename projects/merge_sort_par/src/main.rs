use std::env;
use async_std::task;
use async_std::prelude::FutureExt;
use std::time::SystemTime;
use futures::future::{BoxFuture, FutureExt as FExt};
use std::fs::OpenOptions;
use std::io::prelude::*;


fn split(mut arr : Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let b = arr.split_off(arr.len()/2);
    return (arr, b);
}

fn merge(a : &mut Vec<u32>, b : &mut Vec<u32>) -> Vec<u32> {
    let n = a.len();
    let m = b.len();
    let mut v = vec![0;n+m];

    let mut i = 0;
    let mut j = 0;
    for l in 0..n+m {
	if i >= n {
	    v[l] = b[j];
	    j += 1;
	} else if j >= m {
	    v[l] = a[i];
	    i += 1;
	} else if a[i] > b[j] {
	    v[l] = b[j];
	    j += 1;
	} else {
	    v[l] = a[i];
	    i += 1;
	}
    }

    return v;
}

fn sort(arr : Vec<u32>) -> BoxFuture<'static, Vec<u32>> {

    async move {
	if arr.len() <= 1 {
	    return arr;
	}
	let (a,b) = split(arr);
	let a = sort(a);
	let b = sort(b);
	let a = task::spawn(a);
	let b = task::spawn(b);
	let (mut a,mut b) = a.join(b).await;
	let arr_s = merge(&mut a, &mut b);
	arr_s
    }.boxed()
}

fn sort_seq(arr : Vec<u32>) -> Vec<u32> {
    if arr.len() <= 1 {
	return arr;
    }
    let (a, b) = split(arr);
    let mut a = sort_seq(a);
    let mut b = sort_seq(b);
    let arr_s = merge(&mut a, &mut b);
    return arr_s;
}

#[async_std::main]
async fn main() -> () {
    let args : Vec<String> = env::args().collect();
    let m : u32 = args[1].parse().unwrap_or(1024);
    std::fs::remove_file("sort.dat").unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
	.append(true)
        .open("sort.dat")
	.unwrap();

    file.write_all(b"Length,merge parallel, merge seq, built in\n").unwrap();
    let two : u32 = 2;
    for n in 1..m+1 {
	let n = two.pow(n);
	let mut v : Vec<u32> = Vec::with_capacity(n as usize);
	{ 
	    let mut m = n;
	    for _ in 0..n {
		v.push(m);
		m = m - 1;
	    }
	}

	//Merge Sort (parallel)
	let u = v.clone();
	let start =  SystemTime::now();
	let _u_sorted = task::spawn(sort(u)).await;
	let merge_p_time = start.elapsed().unwrap().as_nanos();

	//Merge Sort (seq)
	let u = v.clone();
	let start =  SystemTime::now();
	let _u_sorted = sort_seq(u);
	let merge_s_time = start.elapsed().unwrap().as_nanos();
	
	//Merge Sort (seq)
	let mut u = v.clone();
	let start =  SystemTime::now();
	u.sort();
	let sort_b_time = start.elapsed().unwrap().as_nanos();

	let s = format!("{},{},{},{}\n", n, merge_p_time, merge_s_time, sort_b_time);
	file.write_all(s.as_bytes()).unwrap();
	println!("{}: ✅", n);

    }
}
