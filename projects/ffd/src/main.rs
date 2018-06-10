use std::cmp::Ordering::*;
use std::cmp::PartialOrd;
use std::ops::Add;

trait Weightable {
    type Output : Add<Output = Self::Output> + PartialOrd + Add<Output = Self::Output>;
    fn weight(&self) -> Self::Output;
}

impl<A: PartialOrd + Add<Output = A> + Copy>  Weightable for A {
    type Output = A;
    fn weight(&self) -> Self::Output {
        *self
    }
}

#[derive(Debug, Clone)]
struct Object<A: Add<Output = A> + PartialOrd + Copy> {
    id: usize,
    weight: A,
}

impl<A: Add<Output = A> + PartialOrd + Copy>  Weightable for Object<A> {
    type Output = A;
    fn weight(&self) -> Self::Output {
        self.weight
    }
}

impl<A: Add<Output = A> + PartialOrd + Copy> Object<A> {
    fn from_weight(x: A) -> Object<A>{
        Object {
            id: 0,
            weight: x,
        }
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

    let objects: Vec<Object<i32>> = vec![Object::from_weight(3),
                                    Object::from_weight(1),
                                    Object::from_weight(4),
                                    Object::from_weight(3),
                                    Object::from_weight(1),
                                    Object::from_weight(1),
                                    Object::from_weight(4),
                                    Object::from_weight(2),
                                    Object::from_weight(3),
                                    Object::from_weight(1),
                                    Object::from_weight(4),
                                    Object::from_weight(2)];
                                     

    let partitions = ffd(objects, 10);
    println!("{:?}", partitions);

    
}

fn ffd<B: Add<Output = B> + PartialOrd,
       A: Weightable<Output = B>>
    (mut objects: Vec<A>, limit_capacity: B) -> Vec<Vec<A>> {
        
    let mut partitions : Vec<Vec<A>> = Vec::new();
    objects.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap_or(Equal));

    for i in objects {
        let mut index = 0;
        
        for partition in &mut partitions {
            let first = partition.pop().unwrap();
            let sum : B = partition.iter().fold(first.weight(), |acc, x| acc + x.weight());
            partition.push(first);

            if sum + i.weight() <= limit_capacity {
                break;
            }
            index = index + 1;
            
        }
        
        if index == partitions.len() {
            partitions.push(Vec::new());
        }
        partitions[index].push(i);


    }
    partitions
}
