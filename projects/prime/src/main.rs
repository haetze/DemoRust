fn main() {
    // for i in 1..100 {
    //     println!("{} is prime? {}", i, is_prime(i));
    //     println!("{} is prime? {}", i, is_prime_2(i));
    // }
    
}

fn is_prime(u: u32) -> bool {
    if u == 0 || u == 1 {
        return false;
    } else {
        for i in 2..u {
            if u % i == 0 {
                return false;
            }
        }
    }
    return true;
}

fn is_prime_2(u: u32) -> bool {
    match u {
        0...1 => false,
        _     => (2..u).all(|x| u % x != 0),
    }
}

fn primes_til(u: u32) -> Vec<u32> {
    let mut v = vec![2];
    for i in 3..u {
        if v.iter().all(|x| i % x != 0) {
            v.push(i);
        }
    }
    v
}
