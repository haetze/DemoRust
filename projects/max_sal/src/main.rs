use std::cmp::Ordering;

/* 
   Brute force approach to maxium Salary problem.
   Provided is a number of positive integers. 
   These are supposed to be arranged in the maxium order.
   For example:
     Provided: 32, 3, 5
     Result: 5332
 */

fn cal(x : u32, ys : &mut Vec<u32>) -> u32 {
    let mut max = x; // This is the minimum that needs to be returned if vec is empty

    // Index needed for removal and re-insertion.
    // Basically cleaning up behind each removal.
    for i in 0..ys.len() {
        // Get the next candidate.
        let y = ys.remove(i);
        
        // Create corresponding strings for correct combining.
        // Otherwise you need to multiply x by 10^n where n is the length of the string
        // representation of y.
        let mut x_str = x.to_string();
        let y_str = y.to_string();
        x_str.push_str(&y_str);

        // Comparing u32 is simpler than strings, therefore the parse back.
        // Unwrap is safe because we only combine strings of digits.
        let combined : u32 = x_str.parse().unwrap();

        // If this next numbers results in a bigger combination, memorize that result.
        let calced = cal(combined, ys);
        if calced > max {
            max = calced;
        }

        // Put the number back where it was.
        ys.insert(i, y);
        
    }
    
    return max;
}


/* 
   Try to implemented a compare operator to find the order without trying each combination.
 */

#[derive(PartialEq)]
struct Num {
    n : u32,
}

impl Num {
    fn new(x : u32) -> Self {
        Num {
            n : x,
        }
    }

    fn len(&self) -> usize {
        self.n.to_string().len() 
    }

    fn size(&self) -> u32 {
        self.n
    }

    fn get_range(&self, left : usize, right : usize) -> Option<u32> {
        let len = self.len();
        if len < left || len < right || right < left {
            return None;
        }
        let string_rep = self.n.to_string();
        let rng : &str = &string_rep[left..right];
        Some(rng.parse().unwrap())
    }

    fn greater_with_idx<'a>(&'a self, other : &'a Self, idx_self : usize, idx_other : usize) -> Option<&'a Self> {
        let shorter;
        let longer;
        let shorter_idx;
        let longer_idx;
        if self.len() - idx_self > other.len() - idx_other{

            shorter = other;
            longer = self;
            shorter_idx = idx_other;
            longer_idx = idx_self;

        } else if self.len() - idx_self < other.len() - idx_other {

            shorter = self;
            longer = other;
            shorter_idx = idx_self;
            longer_idx = idx_other;
            
        } else {

            if self.get_range(idx_self, self.len()) > other.get_range(idx_other, other.len()) {
                return Some(self);
            } else if self.get_range(idx_self, self.len()) < other.get_range(idx_other, other.len()) {
                return Some(other);
            } else {
                return None;
            }

        }

        let shorter_len = shorter.len() - shorter_idx;
        let shorter_size = shorter.get_range(shorter_idx, shorter.len()).unwrap();
        let longer_size = longer.get_range(longer_idx, longer_idx + shorter_len).unwrap();

        if shorter_size > longer_size {
            return Some(shorter);
        } else if longer_size > shorter_size {
            return Some(longer);
        } else {
            return shorter.greater_with_idx(longer, shorter_idx, longer_idx + shorter_len);
        }
    }

    fn cmp(&self, other : &Self) -> Ordering {
        match self.greater_with_idx(other, 0, 0) {
            None => Ordering::Equal,
            Some(greater) if greater == self => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

fn combine(current : u64, next : &Num) -> u64{
    let mut current_str = current.to_string();
    let next_str = next.size().to_string();
    current_str.push_str(&next_str);
    current_str.parse().unwrap()
}


fn main() {

    println!("====================================================");
    // Can't sort by size and 3 needs to be before 32 because 3 > 2
    let v = vec![32,3,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));
    
    let mut sorted_v = v.iter().map(|x| Num::new(*x)).collect::<Vec<Num>>();
    sorted_v.sort_by(|x, y| x.cmp(y));
    let result = sorted_v.iter().rev().fold(0 as u64 , combine);
    println!("Result for {:?} is {}.", v, result);


    println!("====================================================");
    // Can't sort by size and 33 needs to be before 332 because 3 > 2
    let v = vec![332,33,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));
    
    let mut sorted_v = v.iter().map(|x| Num::new(*x)).collect::<Vec<Num>>();
    sorted_v.sort_by(|x, y| x.cmp(y));
    let result = sorted_v.iter().rev().fold(0 as u64 , combine);
    println!("Result for {:?} is {}.", v, result);

    
    println!("====================================================");
    // Sort by size
    let v = vec![9,3,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));

    let mut sorted_v = v.iter().map(|x| Num::new(*x)).collect::<Vec<Num>>();
    sorted_v.sort_by(|x, y| x.cmp(y));
    let result = sorted_v.iter().rev().fold(0 as u64 , combine);
    println!("Result for {:?} is {}.", v, result);


    println!("====================================================");
    // Can't sort by size, order of 55 and 5 doesn't matter
    let v = vec![55,6,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));

    let mut sorted_v = v.iter().map(|x| Num::new(*x)).collect::<Vec<Num>>();
    sorted_v.sort_by(|x, y| x.cmp(y));
    let result = sorted_v.iter().rev().fold(0 as u64 , combine);
    println!("Result for {:?} is {}.", v, result);
    println!("====================================================");

    
}
