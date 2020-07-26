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


fn main() {
    // Can't sort by size and 3 needs to be before 32 because 3 > 2
    let v = vec![32,3,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));

    // Sort by size
    let v = vec![9,3,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));

    // Can't sort by size, order of 55 and 5 doesn't matter
    let v = vec![55,6,5];
    println!("Result for {:?} is {}.", v, cal(0,&mut v.clone()));

    
}
