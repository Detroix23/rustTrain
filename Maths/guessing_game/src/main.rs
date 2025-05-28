// #![allow(unused_doc_comments)]

// Imports

use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Vars
const MSG_LINE_READ_FAIL: str = "(!) - Failed to read the line";

// Main execution
fn main() {

    println!("# Guess the number (game) !");
    let mut game_running: bool = true;
    // Inline game
    /// Choose gamemode
    let mut user_gamemode = String::new();
    io::stdin()
        .read_line(&mut user_gamemode)
        .expect("(!) - Failed to read the line");

    //// Generate random number
    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("**Secret number, chhhh ({secret_number}).**");


    while game_running {
        println!("Input your guess: ");

        let mut user_guess = String::new();

        //// Get user input for a guess
        io::stdin()
            .read_line(&mut user_guess)
            .expect("(!) - Failed to read the line");

        //// Convert user input in u32
        let user_guess: u32 = match user_guess.trim().parse() {
            ///// If succeful, return OK with the resultant number, sent after.
            Ok(num) => num,
            ///// Else, return nothing to user_guess.
            Err(_) => continue,
        };

        println!("You guessed: {}", user_guess);

        //// Compare to secret number
        match user_guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big."),
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                println!("Spot on!");
                game_running = false   
            },
        }
    }

}