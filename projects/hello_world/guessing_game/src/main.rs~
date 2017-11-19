use std::thread;

struct Point {
  x : i32,
  y : i32,
}

impl Point {
    fn new(x : i32, y: i32) -> Point{
        Point {
            x: x,
            y: y,
        }
    }
    
    fn print(&self){
        println!("({}, {})", self.x, self.y);
    }
    
    fn add(&self, p: Point) -> Point{
        Point {
            y:self.y + p.y,
            x:self.x + p.x,
        }
    }
    
        
    
}



fn main() {
    let p1 = Point::new(12, 21);
    let p2 = Point::new(21, 12);
    let p3 = Point::new(23, 32);
    let h = thread::spawn(move || {
        p1.print();
    });
    thread::sleep_ms(100);
    println!("Hello, world!");
    p2.print();
    let sum = p2.add(p3);
    sum.print();
    h.join().unwrap();
    

    
}
