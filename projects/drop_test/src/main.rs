use std::sync::Arc;

#[derive(Debug)]
struct MyRecStruct {
    val : u32,
    inner : Option<Arc<MyRecStruct>>,
}

impl Drop for MyRecStruct {
    fn drop(&mut self) {
        println!("{}", self.val);
    }
}

fn main() {
    let a = MyRecStruct {
        val : 0 ,
        inner : None,
    };

    let c = Arc::new(a);
    let d = c.clone();
    let _b = MyRecStruct {
        val : 1,
        inner : Some(c.clone()),
    };

    let _e = MyRecStruct {
        val : 2,
        inner : Some(d),
    };

    
    println!("{:?}", c);
    
}
