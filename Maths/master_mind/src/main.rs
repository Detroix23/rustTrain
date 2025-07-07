// MASTER MIND
// Rules: you a have a set of, usually, 4 values hidden, taken randomly, or by a fellow human, from a pool of, usually, 8; there can be repetition.
// You have to uncover this set, with a max amount of tries, usually 8, and the hint given on each tries. 
// These hints are how much good and correctly placed values your guessed (⚪), and how much good but incorrectly placed values (🔴).

// Import
use rand::Rng;
use std::io;
use std::collections::HashMap;

use crate::{checks::Hint, search_v1::{combinations_hints, combinations_sets, set_entropy, set_hint_quantities, set_hint_score, SetScore}};

// Basic rules
mod checks;
// Bot solver
mod search_v1;

/// Activate feature-test mode
pub const MODE_TEST: bool = true;
/// Game player. True if the human player want to play and show is natural inferiority, False to let a bot play. 
pub const MODE_GAME_HUMAN: bool = true;


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
/// Graphical - What to print when a value is completely non-existant.
pub const CORRECT_NON: &str = "⚫";

/// Debug - Activate
pub const DEBUG_ACTIVATED: bool = true;

/// Generate a set. Random choices.
pub fn generate_random_set() -> Vec<u32>{
    let mut set_hidden: Vec<u32> = Vec::new();
    while set_hidden.len() < SET_LENGTH {
        set_hidden.push(rand::rng().random_range(1u32..=POOL_SIZE));
    }
    set_hidden
}


/// Execute the game where the human player tries to guess.
pub fn game_manual() -> bool {
    // Generating the set.
    let set_hidden: Vec<u32> = generate_random_set();
    

    if DEBUG_ACTIVATED {println!("- DEBUG - Hidden set: {:?}", set_hidden);}

    // User game.
    println!("\n## Enter your guesses. Seperated by ','. Ex: `1, 2, 3, 4`.");

    let mut guesses: u32 = 1;
    let mut found: bool = false;

    while guesses <= MAX_TRIES && !found {
        // User input.
        let mut user_guess_correct: bool = false;
        let mut user_guess_set_string: String = String::new();
        let mut user_guess_set: Vec<u32> = Vec::new();
        let mut cheat_jojo: bool = false;

        println!("- Guess {}", guesses);
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
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("jojo")) {
                // Cheat - Instant win.
                println!("Cheat - Instant win.");
                cheat_jojo = true;
                user_guess_correct = true;
            } else if DEBUG_ACTIVATED && (user_guess_set_string == String::from("omniscia")) {
                // Cheat - Autocorrect input.
                println!("Cheat - Auto-correct input.");
                user_guess_set = set_hidden.clone();
                user_guess_correct = true;
            } else {
                println!("(!) Warning - Invalid input; Input difference: {}", SET_LENGTH as i32 - user_guess_set.len() as i32);
                user_guess_set_string = String::new();
            }
        }

        // Comparing.
        let comparison_results: Hint = checks::similarities(&set_hidden, &user_guess_set);
        
        // UI
        println!("{} {} {} {} - {:?}", comparison_results.exact, CORRECT_PLACEMENT, comparison_results.exist, CORRECT_VALUE, user_guess_set);

        // End turn.
        found = (comparison_results.exact as usize == SET_LENGTH) | cheat_jojo;

        guesses += 1;
    } 
    println!("\n## Game ended.");
    if found {
        println!("- You win. GG bro.");
    } else {
        println!("- You lost. You ran out of tries. Take the L, bozo.");
    }
    println!("Hidden set was {:?}", set_hidden);
    println!();

    found
}

/// Execute the game for a bot.
/*
pub fn game_robot() -> bool {
    // Generating the set.
    let set_hidden: Vec<u32> = generate_random_set();
    

    println!("- Hidden set, to the bot: {:?}", set_hidden);

    // User game.
    println!("\n## Bot will start entering guesses.");

    let mut guesses: u32 = 1;
    let mut found: bool = false;

    while guesses <= MAX_TRIES && !found {
        // User input.
        let mut user_guess_correct: bool = false;
        let mut user_guess_set_string: String = String::new();
        let mut user_guess_set: Vec<u32> = Vec::new();
        println!("- Guess {}", guesses);

        // Bot guess
        //// Undef function.

        // Comparing.
        let comparison_results: [u32; 2] = checks::similarities(&set_hidden, &user_guess_set);
        
        // UI
        println!("{} {} {} {} - {:?}", comparison_results[0], CORRECT_PLACEMENT, comparison_results[1], CORRECT_VALUE, user_guess_set);

        // End turn.
        found = comparison_results[0] as usize == SET_LENGTH;

        guesses += 1;
    } 
    println!("\n## Game ended.");
    if found {
        println!("- You win. GG bro.");
    } else {
        println!("- You lost. You ran out of tries. Take the L, bozo.");
    }
    println!("Hidden set was {:?}", set_hidden);
    println!();

    found
}
*/

/// Enter in a state to test features and function. Disable playing.
/// Edit directly in the code
fn mode_test() {
    println!("## Feature-test mode");

    println!("### Recursion test");
    let _combinations_sets: Vec<Vec<u32>> = combinations_sets();
    let _combinations_hint: Vec<Hint> = combinations_hints();
    /*
    println!("Combination of hints:");
    for hint in &_combinations_hint {
        println!("- {}{} {}{} {}{}", hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }
    println!("q: {}", _combinations_hint.len());
    */
    /*
    println!("Combinations of sets: {:?}. # {}", _combinations_sets, _combinations_sets.len());
    */

    println!("\n### Hints quantities and probabilities");
    let _set: Vec<u32> = vec![1, 1, 1, 1];
    println!("Set = {:?}", _set);
    // Print hint quantities
    let _hint_quantities: HashMap<Hint, u32> = set_hint_quantities(&_set, &_combinations_sets, &_combinations_hint);
    println!("Hint quantities:");
    for (hint, weight) in &_hint_quantities {
        println!("- q = {} -> {}{} {}{} {}{}", weight, hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }
    // println!("q: {}", _hint_quantities.len());
    // Hint proba
    println!("Hint probability:");
    let _hint_score: HashMap<Hint, SetScore> = set_hint_score(_hint_quantities);
    for (hint, score) in &_hint_score {
        println!("- p = {:.2}%; b = {:.2}bits -> {}{} {}{} {}{}", score.probability * 100f64, score.bits,hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }

    // Set overhaul score
    println!("Entropy:");
    let _entropy = set_entropy(_hint_score);
    println!("- E = {:.5}bits", {_entropy});

}

/// Run the game, according to settings, debug and player.
pub fn main() {
    println!("# MASTER MIND");

    println!("## Initializing");
    if POOL_SIZE < 16 {
        print!("- Pool of values: 1");
        for value in 2..=POOL_SIZE {
            print!(", {value}");
        }
        println!();
        } else {
        println!("- Pool of values (inclusive): 1 .. {}", POOL_SIZE);
    }
    println!("- Set length: {}", SET_LENGTH);
    println!("- Maximum tries: {}", MAX_TRIES);

    if MODE_TEST {
        mode_test();
    } else if MODE_GAME_HUMAN {
        game_manual();
    } else if !MODE_GAME_HUMAN {
        // game_robot()
    } else {
        println!("(X) - No mode selected. Exitting.");
    }

}