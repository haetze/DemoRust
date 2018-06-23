use std::iter::Iterator;

trait IsInfiniteSequence: Iterator{}

struct LinearCongruentialMethod {
    m: i32,
    a: i32,
    c: i32,
    random_numbers_til_now: Vec<i32>,
}

impl LinearCongruentialMethod{
    fn new(m: i32, a: i32, c: i32, seed: i32) -> Self {

        LinearCongruentialMethod {
            m: m,
            a: a,
            c: c,
            random_numbers_til_now: vec![seed],
        }

    }
}

impl Iterator for LinearCongruentialMethod {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let x_n = self.random_numbers_til_now.pop().unwrap();
        let x_n_plus_1 = (self.a * x_n + self.c) % self.m;
        self.random_numbers_til_now.push(x_n);
        self.random_numbers_til_now.push(x_n_plus_1);
        Some(x_n_plus_1)
    }
}

impl IsInfiniteSequence for LinearCongruentialMethod {}

struct FibGen {
    m: i32,
    random_numbers_til_now: Vec<i32>,
}

impl FibGen{
    fn new(m: i32, seed_1: i32, seed_2: i32) -> Self {
        
        FibGen {
            m: m,
            random_numbers_til_now: vec![seed_1, seed_2],
        }

    }
}


impl Iterator for FibGen {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let x_n = self.random_numbers_til_now.pop().unwrap();
        let x_n_minus_1 = self.random_numbers_til_now.pop().unwrap();
        let x_n_plus_1 = (x_n + x_n_minus_1) % self.m;
        self.random_numbers_til_now.push(x_n_minus_1);
        self.random_numbers_til_now.push(x_n);
        self.random_numbers_til_now.push(x_n_plus_1);
        Some(x_n_plus_1)
    }
}
impl IsInfiniteSequence for FibGen{}

struct Shuffler<A: Iterator<Item=i32> + IsInfiniteSequence,
                B: Iterator<Item=i32> + IsInfiniteSequence> {
    m: i32,
    rnd_gen_1: A,
    rnd_gen_2: B,
}

impl<A: Iterator<Item=i32> + IsInfiniteSequence,
     B: Iterator<Item=i32> + IsInfiniteSequence> Shuffler<A, B> {
    fn new(rnd_gen_1: A, rnd_gen_2: B, m: i32) -> Self {
        Shuffler {
            rnd_gen_1: rnd_gen_1,
            rnd_gen_2: rnd_gen_2,
            m: m,
        }
    }
}

impl<A: Iterator<Item=i32> + IsInfiniteSequence,
     B: Iterator<Item=i32> + IsInfiniteSequence> Iterator for Shuffler<A, B>{
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let x_n = self.rnd_gen_1.next().unwrap();
        let y_n = self.rnd_gen_2.next().unwrap();
        let z_n = (x_n - y_n) % self.m;
        if z_n < 0 {
            Some(z_n * -1)
        }else{
            Some(z_n)
        }
    }

}

impl<A: Iterator<Item=i32> + IsInfiniteSequence,
     B: Iterator<Item=i32> + IsInfiniteSequence> IsInfiniteSequence for Shuffler<A, B> {}



struct GeneratorWithFn{
    f: Box<Fn(&mut Vec<i32>) -> i32>,
    v: Vec<i32>,
}

impl GeneratorWithFn{
    fn new(f: Box<Fn(&mut Vec<i32>) -> i32>, v: Vec<i32>) -> Self{
        GeneratorWithFn {
            f: f,
            v: v,
        }
    }
    
    fn new_fib(m: i32, seed_1: i32, seed_2: i32) -> Self {
         GeneratorWithFn::new(Box::new(move |v: &mut Vec<i32>| {
             let x = v.pop().unwrap();
             let y = v.pop().unwrap();
             let z = (x + y) % m;
             v.push(y);
             v.push(x);
             v.push(z);
             return z;
         }),vec![seed_1, seed_2])
    }

    fn new_linear_con(m: i32, a: i32, c: i32, seed: i32) -> Self {
        GeneratorWithFn::new(Box::new(move |v: &mut Vec<i32>| {
            let x_n = v.pop().unwrap();
            let x_n_plus_1 = (a * x_n + c) % m;
            v.push(x_n);
            v.push(x_n_plus_1);
            x_n_plus_1
        }), vec![seed])
    }
}

impl Iterator for GeneratorWithFn {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        use std::mem::swap;
        let mut v = Vec::new();
        swap(&mut v, &mut self.v);
        let z = (self.f)(&mut v);
        swap(&mut v, &mut self.v);
        Some(z)
            
    }
}

impl IsInfiniteSequence for GeneratorWithFn{}
    

fn print_n_for_iter<A: Iterator<Item=i32> + IsInfiniteSequence>(mut iter: A, n: i32, name: String){
    println!("========={}=========", name);
    for i in 0..n {
        println!("{}:\t{}", i, iter.next().unwrap());
    }
}


fn main() {
    let m_lin = 100;
    let a = 7;
    let c = 8;
    let seed_lin = 9;    
    let random_gen_1 = LinearCongruentialMethod::new(m_lin, a, c, seed_lin);
    print_n_for_iter(random_gen_1, 100, "LinearCongruentialMethod".to_string());

    let m_fib = 100;
    let seed_1 = 7;
    let seed_2 = 8;
    let random_gen_2 = FibGen::new(m_fib, seed_1, seed_2);
    print_n_for_iter(random_gen_2, 100, "FibGen".to_string());

    let rnd_gen_1 = LinearCongruentialMethod::new(12343, 84, 56, 43);
    let rnd_gen_2 = LinearCongruentialMethod::new(1243, 284, 156, 73);
    let random_gen_3 = Shuffler::new(rnd_gen_1, rnd_gen_2, 321);
    print_n_for_iter(random_gen_3, 100, "Shuffler".to_string());

    let random_gen_4 = GeneratorWithFn::new_fib(m_fib, seed_1, seed_2);
    print_n_for_iter(random_gen_4, 100, "FibGenFn".to_string());

    let random_gen_1 = GeneratorWithFn::new_linear_con(m_lin, a, c, seed_lin);
    print_n_for_iter(random_gen_1, 100, "LinearCongruentialMethodFn".to_string());
    
}
