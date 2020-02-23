use std::collections::BTreeSet;

#[derive(std::cmp::PartialEq, std::cmp::Eq,
         std::cmp::PartialOrd, std::cmp::Ord,
         std::fmt::Debug)]
enum Exec {
    U8(u8),
    U32(u32),
    U64(u64),
}

use Exec::*;

trait Typable {
    fn type_of(&self) -> Exec;
}

impl Typable for u8 {
    fn type_of(&self) -> Exec {
        U8(self.clone())
    }
}

impl Typable for u32 {
    fn type_of(&self) -> Exec {
        U32(self.clone())
    }
}

impl std::cmp::PartialEq<dyn Typable> for dyn Typable {
    fn eq(&self, other: &dyn Typable) -> bool {
        self.type_of() == other.type_of()
    }
}

impl std::cmp::Eq for dyn Typable { }

impl std::cmp::PartialOrd<dyn Typable> for dyn Typable {
    fn partial_cmp(&self, other: &dyn Typable) -> Option<std::cmp::Ordering> {
        self.type_of().partial_cmp(&other.type_of())
    }
}

impl std::cmp::Ord for dyn Typable {
    fn cmp(&self, other : &Self) -> std::cmp::Ordering{
        self.type_of().cmp(&other.type_of())
    }
}

impl std::fmt::Debug for dyn Typable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}", self.type_of())
    }
}

fn main() {
    let mut set : BTreeSet<Box<dyn Typable>> = BTreeSet::new();
    set.insert(Box::new(6 as u32));
    if set.contains(&(Box::new(7 as u32) as Box<dyn Typable>)) {
        set.insert(Box::new(1 as u8));
    } else {
        set.insert(Box::new(0 as u8));
    }
    
    println!("{:?}", set);
}
