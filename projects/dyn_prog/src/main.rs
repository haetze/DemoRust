fn fib(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1)+fib(n-2)
    }
}

fn fib_dyn(n:u64) -> u64 {
    let mut fib_seq = [0;2];
    fib_seq[1] = 1;
    for _ in 0..n {
        let a = fib_seq[1];
        let b = fib_seq[0];
        let c = a+b;
        fib_seq[0] = a;
        fib_seq[1] = c;
    }
    fib_seq[0]
}

fn main() {
    for x in 0..40 { 
        let fib_x = fib_dyn(x);
        println!("{}", fib_x);
    
        let fib_x = fib(x);
        println!("{}", fib_x);

        println!("--------------------");
    }

}
