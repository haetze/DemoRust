use std::collections::HashMap;

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

fn value(objects: &Vec<(usize, usize)>, k: usize) -> usize {
    value_rec(objects, 0, k)
}

fn value_rec(objects: &Vec<(usize, usize)>, i: usize, k: usize) -> usize {
    let n = objects.len() - 1;
    let (weight, value) = objects[i];
    if i == n {
        if weight > k {
            return 0;
        } else { 
            return value;
        }
    } else if i < n {
        let a = value_rec(objects, i+1, k);
        if weight > k {
            return a;
        } else {
            let b = value_rec(objects, i+1, k-weight) + value;
            if a < b {
                return b;
            } else {
                return a;
            }
        }
    }
    panic!("No Solution");
}

fn value_dyn(objects: &Vec<(usize, usize)>, k: usize) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    value_dyn_calc(objects, 0, k, objects.len()-1, &mut map)
}

fn value_dyn_calc(objects: &Vec<(usize, usize)>,
                  i: usize,
                  k: usize,
                  n: usize,
                  map: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (i,k);
    match map.get(&key) {
        Some(result) => result.clone(),
        None => {
            let (weight, value) = objects[i];
            if i == n {
                if weight > k {
                    map.insert(key, 0);
                    return 0;
                } else {
                    map.insert(key, value);
                    return value;
                }
            } else if i < n {
                let a = match map.get(&(i+1, k)) {
                    None => value_dyn_calc(objects,
                                           i+1,
                                           k,
                                           n,
                                           map),
                    Some(n) => n.clone(),
                };
                if weight > k {
                    map.insert(key, a);
                    return a;
                } else {
                    let b = match map.get(&(i+1, k- weight)) {
                        None => value_dyn_calc(objects,
                                               i+1,
                                               k-weight,
                                               n,
                                               map) + value,
                        Some(n) => n.clone() + value,
                    };
                    if a > b {
                        map.insert(key, a);
                        return a;
                    } else {
                        map.insert(key, b);
                        return b;
                    }
                }
                
            }
            panic!("No Solution");                    
        },
    }
}


fn main() {
    for x in 0..40 { 
        let fib_x = fib_dyn(x);
        println!("{}", fib_x);
    
        let fib_x = fib(x);
        println!("{}", fib_x);

        println!("--------------------");
    }
    let dinge = vec![(2,2)
                     ,(2,3)
                     ,(6,5)
                     ,(5,4)
                     ,(4,6)];
    let kapazitaet = 10;
    println!("Rucksack-----------------");
    println!("Dinge: {:?}", dinge);
    let sol = value(&dinge, kapazitaet);
    println!("Loesung: {}", sol);
    println!("Dinge: {:?}", dinge);
    let sol = value_dyn(&dinge, kapazitaet);
    println!("Loesung: {}", sol);
    

}
