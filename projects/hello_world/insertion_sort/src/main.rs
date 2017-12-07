#![allow(unused_must_use)]

extern crate colored;
extern crate rand;

use rand::Rng;
use colored::*;
use std::thread;

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
    let i = 5000;

    let handle_insertion_sort = thread::spawn(move || {
        let mut v1 = random_vec(i, i);
        v1.reverse();
        {
            let v2 = v1.insertion_sort();
            assert!(v2.is_sorted());
        
        }
        v1.insertion_sort_mut();
        assert!(v1.is_sorted());

        println!("{}", "Insertion Sort Success".green().bold());
    });

    let handle_quick_sort = thread::spawn(move || {
        let mut v1 = random_vec(i, i);
        v1.reverse();
        {
            let v2 = v1.quick_sort();
            assert!(v2.is_sorted());
        }
        v1.quick_sort_mut();
        assert!(v1.is_sorted());
        println!("{}", "Quick Sort Success".green().bold());
    });

    let handle_selection_sort = thread::spawn(move || {
        let mut v1 : Vec<i32>= random_vec(i, i);
        v1.reverse();
        {
            let v2 = v1.selection_sort();
            assert!(v2.is_sorted());
        }
        v1.selection_sort_mut();
        assert!(v1.is_sorted());
        println!("{}", "Selection Sort Success".green().bold());
    });


    handle_quick_sort.join();
    handle_selection_sort.join();
    handle_insertion_sort.join();
    
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
            print!("{}\n", "Not sorted!".red().bold());
        }else{
            print!("{}\n", "Sorted!".green().bold());
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

    
