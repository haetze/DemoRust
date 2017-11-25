use std::thread;

fn main() {
    let n = 100;
    let mut handler_vec = Vec::new();
    for i in 0..n{
        handler_vec.push(thread::spawn(move || {
            println!("From child:{}", i);
            i*2
        }));
    }

    for child in handler_vec{
        let i = child.join().unwrap();
        println!("From main:{}", i);
    }
                         
}
