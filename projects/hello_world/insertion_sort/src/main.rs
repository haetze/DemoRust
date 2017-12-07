extern crate colored;
extern crate rand;

use rand::Rng;
use colored::*;

trait Sort<A:PartialOrd> {
    fn insertion_sort_mut(&mut self);
    fn insertion_sort(&self) -> Vec<&A>;
    fn quick_sort_mut(&mut self);
    fn quick_sort(&self) -> Vec<&A>;
    fn selection_sort_mut(&mut self);
    fn selection_sort(&self) -> Vec<&A>;
    fn is_sorted(&self) -> bool;
    fn check_sorted_output(&self);

}

fn random_vec(n: i32, limit:i32) -> Vec<i32>{
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..n{
        let m: i32 = rng.gen();
        v.push(m % limit);
    }
    return v;
}



fn main() {
    let i = 500;
    println!("\n\n\n+++++++++++++++\t Insertion Sort\t+++++++++++++++\n");
    {
        let mut v1 = random_vec(i, i);
        v1.reverse();
        println!("Original Vector: {:?}\n", v1);
        {
            let v2 = v1.insertion_sort();
            println!("Sortet Vector(copy, insertion sort):{:?}\n", v2);
            v2.check_sorted_output();
            println!("Original Vector:{:?}\n", v1);
        }
        v1.insertion_sort_mut();
        println!("Original Vector (sorted, insertion sort):{:?}\n", v1);
        v1.check_sorted_output();
    }
    println!("\n\n\n+++++++++++++++\t Quick Sort\t+++++++++++++++\n");
    {
        let mut v1 = random_vec(i, i);
        v1.reverse();
        println!("Original Vector: {:?}\n", v1);
        {
            let v2 = v1.quick_sort();
            println!("Sortet Vector(copy, quick sort):{:?}\n", v2);
            v2.check_sorted_output();
            println!("Original Vector:{:?}\n", v1);
        }
        v1.quick_sort_mut();
        println!("Original Vector (sorted, quick sort):{:?}\n", v1);
        v1.check_sorted_output();
    }
    println!("\n\n\n+++++++++++++++\t Selection Sort\t+++++++++++++++\n");
    {
        let mut v1 : Vec<i32>= random_vec(i, i);
        v1.reverse();
        println!("Original Vector: {:?}\n", v1);
        {
            let v2 = v1.selection_sort();
            println!("Sortet Vector(copy, selection sort):{:?}\n", v2);
            v2.check_sorted_output();
            println!("Original Vector:{:?}\n", v1);
        }
        v1.selection_sort_mut();
        println!("Original Vector (sorted, selection sort):{:?}\n", v1);
        v1.check_sorted_output();
    }
    
}



impl<A:PartialOrd> Sort<A> for [A]{

    fn is_sorted(&self) -> bool{
        let mut pre = 0;
        for i in 1..self.len(){
            if self[i] < self[pre]{
                return false;
            }
            pre = pre + 1;
        }
        return true;
    }

    fn check_sorted_output(&self){
        if !self.is_sorted(){
            println!("{}\n", "Not sorted!".red().bold());
        }else{
            println!("{}\n", "Sorted!".green().bold());
        }
    }

               
            

    fn selection_sort_mut(&mut self){
        let n = self.len();
        let mut links = 0;
        while links < n {
            let mut min = links;
            for i in (links+1)..n {
                if self[i] < self[min] {
                    min = i; 
                }
            }
            self.swap(min, links);
            links = links + 1;
        }
    }

    fn selection_sort(&self) -> Vec<&A>{
        let mut v = Vec::new();
        for n in self {
            v.push(n);
        }
        v.selection_sort_mut();
        return v;
    }

    
    fn quick_sort_mut(&mut self){
        if self.len() <= 1{
            return;
        }else{
            let (mut pivot, mut hi) = (0, self.len() - 1);
            for _ in 0..self.len() - 1{
                if self[pivot] < self[pivot+1]{
                    self.swap(pivot+1, hi);
                    hi = hi - 1;
                } else {
                    self.swap(pivot, pivot + 1);
                    pivot = pivot + 1;
                }

            }
            self[..pivot].quick_sort_mut();
            self[pivot+1..].quick_sort_mut();
        }
       
    }

    fn quick_sort(&self) -> Vec<&A>{
        let mut v = Vec::new();
        for n in self{
            v.push(n);
        }
        v.quick_sort_mut();
        return v;
    }

  
        
    
    fn  insertion_sort_mut(&mut self){
        let mut i = 1;
        while i < self.len() {
            let mut j = i;
            while j > 0 && self[j-1] > self[j] {
                self.swap(j-1, j);
                j = j - 1;
            }
            i = i + 1;
        }
    }
    
    fn  insertion_sort(&self) -> Vec<&A>{
        let mut v = Vec::new();
        for n in self{
            v.push(n);
        }
        v.insertion_sort_mut();
        return v;

    }
}

    
