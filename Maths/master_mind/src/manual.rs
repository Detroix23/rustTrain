// MASTER MIND
// Human gameplay.

use std::io;

use crate::{checks::{self, Hint}, generate_random_set, CORRECT_PLACEMENT, CORRECT_VALUE, DEBUG_ACTIVATED, MAX_TRIES, SET_LENGTH};


/// Execute the game where the human player tries to guess.
pub fn game_manual(set_hidden: Vec<u32>) -> bool {
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