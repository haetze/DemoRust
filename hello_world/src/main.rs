//use std::thread;

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
    let p = Point::new(12, 21);
    let p2 = Point::new(21, 12);
    let h = thread::spawn(p.print());
    //thread::sleep_ms(100);
    println!("Hello, world!");
    p.print();
    let sum = p.add(p2);
    sum.print();
    //h.join().unwrap();

    
}
