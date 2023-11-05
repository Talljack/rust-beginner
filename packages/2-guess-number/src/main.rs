use rand::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("Guess the number! 1-1000");
    // Generate a random number between 1 and 1000
    let secret_num = thread_rng().gen_range(1..=1000);
    loop {
        println!("{}", "Please input your guess.".green());
        // Create a mutable variable to store the user input
        let mut guess_number = String::from("");
        // Read the user input
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");
        // Convert the user input to a number
        let guess_number = match guess_number.trim().parse::<i32>() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                return ();
            },
        };
        println!("You guessed: {}", guess_number.to_string().yellow());
        match guess_number.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!!".red()),
            Ordering::Greater => println!("{}", "Too big!!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            },
        }
    }
}