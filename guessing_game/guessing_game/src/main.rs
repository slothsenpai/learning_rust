extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101); // Generate the secret number, store in a variable
    
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); // Sets a new variable as a mutable string type

        io::stdin().read_line(&mut guess) // Gets user input and stores it whereever the mutable reference to guess points
            .expect("Failed to read line"); // println! returns a Result which will be the number of bytes
                                            // Taken up by user input if it is Ok, if it is Err, .expect handles
                                            // the error and prints what is passed as the parameter

        let guess: u32 =  match guess.trim().parse() { // Error handling, if match returns Ok guess will receive that
            Ok(num) => num,                            // value and store it, otherwise we just continue the loop
            Err(_) => continue,                        // until we receive good data
        };

        println!("You guessed: {}", guess); // similar to printf, {} holds whatever variables you pass at the end

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
