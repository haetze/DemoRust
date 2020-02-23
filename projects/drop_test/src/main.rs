use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SET : Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
}
    
#[derive(Debug, std::hash::Hash, std::cmp::PartialEq, std::cmp::Eq)]
struct MyRecStruct {
    val : u32,
    inner : Option<Arc<MyRecStruct>>,
}

impl Drop for MyRecStruct {
    fn drop(&mut self) {
        println!("{}", self.val);
        SET.lock().unwrap().insert(self.val);
    }
}

fn main() {
    let mut test_set = HashSet::new();

    {
        let a = MyRecStruct {
            val : 0 ,
            inner : None,
        };

        let c = Arc::new(a);
        let d = c.clone();
        let _b = MyRecStruct {
            val : 1,
            inner : Some(c),
        };
     
        let e = MyRecStruct {
            val : 2,
            inner : Some(d),
        };
        test_set.insert(e);
        test_set;
    }
    println!("{:?}", SET.lock().unwrap());
}
