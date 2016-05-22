extern crate rand;




use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    //declaration
    let secret_number = rand::thread_rng().gen_range(1, 101); 

    println!("Guess the number.");
    
    //println!("The secret number is {}", secret_number);

    loop{

        let mut guess = String::new(); // creates a new mult sring object

        println!("Please enter your guess.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        //print!("You guessed {}", guess);

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

    }
}
