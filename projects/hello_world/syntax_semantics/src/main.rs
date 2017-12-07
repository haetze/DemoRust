struct Point {

    x :i32,
    y :i32
}

struct PointRef<'a>{
    x: &'a mut i32,
    y: &'a mut i32
}

//tuple struct
struct Color(i32, i32, i32);

fn main() {

    let x = 100; //imutable variable
    //let x = 1; fails at compile time
    let mut y = 100;
    println!("{}, {}", x, y);
    y = x * 2; //succseeds, because y is mutable
    println!("{}, {}", x, y);
    print_number(x); // function call
    print_number(add_two_number(x, y));
    
    
    //bools
    let t = true;
    let f = false;
    println!("{}, {}", f , t);


    //arrays
    let mut a  = [1, 2, 2, 3];
    println!("{}", a[1]);
    a[1] = 100;
    println!("{}", a[1]);

    //vector
    let mut v = vec!['1', '2', '3', '4'];
    for a in &v {
        println!("{}", a);
    }

    add_element_to_vec(&mut v, 'a');
    
    println!("");
    for a in &v {
        println!("{}", a);
    }

    let mut a = 42;
    let mut c = 21;
    {
        let mut b = &mut a;
        println!("{}", b);
        *b = 43;
        println!("{}", b);
        println!("{}", c);
        b = &mut c;
        *b = 43;
        println!("{}", b);
    }
    

    println!("{}", a);


    //struct

    let point_a = Point{x:0, y:0};
    let mut point_b = Point{x:5, y: 0};

    println!("({},{})", point_b.x, point_b.y);
    {
        let point_ref = PointRef{x: &mut point_b.x,
                                 y: &mut point_b.y};

        println!("({},{})", point_a.x, point_a.y);
        *point_ref.x = 10;
    
    }
    println!("({},{})", point_b.x, point_b.y);
    
    point_b.x = 12;
    println!("({},{})", point_b.x, point_b.y);
    

    //tuple struct

    let black = Color(0,0,0);

    let Color(red,_,_) = black;

    println!("Red part of black {}", red);


    let mut f : Vec<i32> = (0..100).collect();

    println!("{:?}", f);

    ff(&mut f[..50]);

    println!("{:?}", f);
        
        
        
}


fn ff(i:&mut [i32]){
    for n in 0..i.len(){
        i[n] = 0;
    }
}
//function declaration
fn print_number(x :i32){
    println!("{}", x);
}

fn add_element_to_vec<A>(ass :&mut Vec<A>, x: A){
    ass.push(x);
}


//function declaration with return type
fn add_two_number(x :i32, y :i32) -> i32{
    x + y
    //return x+y is considered poor style, except early returns
}

/*
fn diverge () -> ! {
    panic!("This function never returns");
}*/

