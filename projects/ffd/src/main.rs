use std::cmp::Ordering::*;
use std::cmp::PartialOrd;
use std::ops::Add;

trait Weightable {
    type Output : Add<Output = Self::Output> + PartialOrd;
    fn weight(&self) -> Self::Output;
}

impl<A: PartialOrd + Add<Output = A> + Clone + Copy>  Weightable for A {
    type Output = A;
    fn weight(&self) -> Self::Output {
        self.clone()
    }
}

fn main() {
    let objects: Vec<i32> = vec![3, 1, 4,
                                 3, 1,
                                 1, 4, 2,
                                 3, 1, 4, 2];
                                     

    let partitions = ffd(objects, 10);
    println!("{:?}", partitions);

    let objects: Vec<f32> = vec![0.3, 0.1, 0.4,
                                 0.3, 0.1,
                                 0.1, 0.4, 0.2,
                                 0.3, 0.1, 0.4, 0.2];
                                     

    let partitions = ffd(objects, 1.0);
    println!("{:?}", partitions);

    
}

fn ffd<B: Add<Output = B> + PartialOrd,
       A: Weightable<Output = B> + Clone + Copy>
    (mut objects: Vec<A>, limit_capacity: B) -> Vec<Vec<A>> {
        
    let mut partitions : Vec<Vec<A>> = Vec::new();
    objects.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap_or(Equal));

    for i in objects {
        let mut added = false;
        for partition in &mut partitions {
            let first = partition.iter().nth(0).unwrap().clone();
            let sum : B = partition.iter().skip(1).fold(first.weight(), |acc, x| acc + x.weight());
            
            if sum + i.weight() <= limit_capacity {
                partition.push(i);
                added = true;
                break;
            }
            
        }
        
        if !added {
            let mut p = Vec::new();
            p.push(i);
            partitions.push(p);
        }

    }
    partitions
}
