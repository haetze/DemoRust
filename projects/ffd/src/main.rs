fn main() {
    let objects: Vec<u32> = vec![3, 1, 4,
                                 3, 1,
                                 1, 4, 2,
                                 3, 1, 4, 2];
                                     

    let partitions = ffd(objects, 10);
    println!("{:?}", partitions);
    
}

fn ffd(mut objects: Vec<u32>, limit_capacity: u32) -> Vec<Vec<u32>> {
    let mut partitions : Vec<Vec<u32>> = Vec::new();
    objects.sort_by(|a, b| b.cmp(a));

    for i in objects {
        let mut added = false;
        for partition in &mut partitions {
            let sum : u32 = partition.iter().sum();
            if sum + i <= limit_capacity {
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
