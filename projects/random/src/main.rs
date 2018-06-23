use std::iter::Iterator;

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



fn print_n_for_iter<A: Iterator<Item=i32>>(mut iter: A, n: i32, name: String){
    println!("========={}=========", name);
    for i in 0..n {
        println!("{}:\t{}", i, iter.next().unwrap());
    }
}


fn main() {
    let random_gen_1 = LinearCongruentialMethod::new(100, 7, 8, 9);
    print_n_for_iter(random_gen_1, 100, "LinearCongruentialMethod".to_string());

    let random_gen_2 = FibGen::new(100, 7, 8);
    print_n_for_iter(random_gen_2, 100, "FibGen".to_string());
}
