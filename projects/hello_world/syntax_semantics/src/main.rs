


fn main() {

    let x = 100; //imutable variable
    //let x = 1; fails at compile time
    let mut y = 100;

    //bools
    let t = true;
    let f = false;


    //arrays
    let a  = [1, 2, 2, 3];
    println!("{}, {}", x, y);

    y = x * 2; //succseeds, because y is mutable
    
    println!("{}, {}", x, y);


    print_number(x); // function call

    print_number(add_two_number(x, y));

    println!("{}, {}", f , t);

    println!("{}", a[1]);

    //diverge();
    
}

//function declaration
fn print_number(x :i32){
    println!("{}", x);
}


//function declaration with return type
fn add_two_number(x :i32, y :i32) -> i32{
    x + y
    //return x+y is considered poor style, except early returns
}


fn diverge () -> ! {
    panic!("This function never returns");
}

