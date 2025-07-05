// MASTER MIND
// Rules: you a have a set of, usually, 4 values hidden, taken randomly, or by a fellow human, from a pool of, usually, 8; there can be repetition.
// You have to uncover this set, with a max amount of tries, usually 8, and the hint given on each tries. 
// These hints are how much good and correctly placed values your guessed (⚪), and how much good but incorrectly placed values (🔴).

// Import
use rand::Rng;
use std::io;

mod checks;

/// Define how many "colors" values there is.
pub const POOL_SIZE: u32 = 8u32;
/// Define the length of the hidden set.
pub const SET_LENGTH: usize = 4usize;
/// Define the maximum amount of tries availabes
pub const MAX_TRIES: u32 = 8u32;

/// Graphical - What to print when guess is correct and well placed.
pub const CORRECT_PLACEMENT: &str = "⚪";
/// Graphical - What to print when a value is good.
pub const CORRECT_VALUE: &str = "🔴";



pub fn main() {
    println!("# MASTER MIND");
    println!("Values: POOL_SIZE = {POOL_SIZE}, SET_LENGTH = {SET_LENGTH}, MAX_TRIES = {MAX_TRIES}");

    // Generating the set.
    let mut set_hidden: Vec<u32> = Vec::new();
    while set_hidden.len() < SET_LENGTH {
        set_hidden.push(rand::rng().random_range(1u32..=POOL_SIZE));
    }

    println!("set: {:?}", set_hidden);

    // User game.
    println!("## Enter your guesses seperated by ','. Ex: *1, 2, 3, 4*.");

    let mut guesses: u32 = 1;
    let mut found: bool = false;

    while guesses <= MAX_TRIES && !found {
        // User input.
        let mut user_guess_correct: bool = false;
        let mut user_guess_set_string: String = String::new();
        let mut user_guess_set: Vec<u32> = Vec::new();
        let mut cheat_jojo: bool = false;

        println!("Guess {}", guesses);
        while !user_guess_correct {
            // Read
            io::stdin()
                .read_line(&mut user_guess_set_string)
                .expect("(!) Warning - Couldn't read line.");

            user_guess_set_string = user_guess_set_string.trim().to_string();
            // Convert to vector
            user_guess_set = checks::convert_input_set(user_guess_set_string.clone());
            
            
            // Validate input
            if user_guess_set.len() == SET_LENGTH {
                user_guess_correct = true;
            } else if user_guess_set_string == String::from("jojo") { 
                // Cheat - Instant win.
                println!("Cheat - Instant win");
                cheat_jojo = true;
                user_guess_correct = true;
            } else if user_guess_set_string == String::from("omniscia") {
                // Cheat - Autocorrect input.
                user_guess_set = set_hidden.clone();
                user_guess_correct = true;
            } else {
                println!("(!) Warning - Invalid input; please type again.");
                user_guess_set_string = String::new();
            }
        }
        println!("set: {:?}", user_guess_set);

        // Comparing.
        let comparison_results: Vec<u8> = checks::similarities(set_hidden.clone(), user_guess_set);
        println!("comparison: {:?}", comparison_results);

        // End turn.
        found = (comparison_results.iter().sum::<u8>() as usize == SET_LENGTH * 2usize) | cheat_jojo;

        guesses += 1;
    } 
    println!();
    if found {
        println!("GG bro.");
    } else {
        println!("Take the L, bozo. You ran out of tries. Solution was {:?}", set_hidden);
    }


}