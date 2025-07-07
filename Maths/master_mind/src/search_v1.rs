// MASTER MIND
// Bot search.

use std::hint;

// Import from Main.
use crate::{POOL_SIZE, SET_LENGTH};
// Import from Checks.
use crate::{checks::{similarities, Hint}};

pub struct WeightedHint {
	pub hint: Hint,
	pub weight: u32,
}

/// Generate all possible combinations of set, according to main constant's.
/// # combinations = pool_size ^ set_length.
pub fn combinations_sets() -> Vec<Vec<u32>> {
	fn combinations_sets_inner(mut iter_global: usize) -> Vec<Vec<u32>> {
		let mut combinations_send: Vec<Vec<u32>> = Vec::new();

		if iter_global < SET_LENGTH {
			iter_global += 1;
			let combinations_received: Vec<Vec<u32>> = combinations_sets_inner(iter_global);
			// println!("\n====================Iter {iter_global} - NORMAL MODE=======================");
			
			for combination in combinations_received {
				for iter_local in 1..=POOL_SIZE {
					let mut combination_temp: Vec<u32> = combination.to_vec();
					combination_temp.push(iter_local);
					combinations_send.push(combination_temp);
				}	
			}
		} else {
			// println!("\n====================Iter {iter_global} - INIT MOD=========================");
			for iter_local in 1..=POOL_SIZE {
				let combination: Vec<u32> = vec![iter_local];
				combinations_send.push(combination);
			}
		}
		
		// print!("{:?} ", combinations_send);

		combinations_send
	}

	let combinations: Vec<Vec<u32>> = combinations_sets_inner(1);

	combinations
}

/// Generate all possible combinations of hints given, *Exact* (2), *Exist* (1), *Non-exist* (0).
/// # combinations = 3 ^ set_length.
pub fn combinations_hints() -> Vec<Hint> {
	// Generate all combinations
	fn combinations_sets_inner(mut iter_global: usize) -> Vec<Vec<u32>> {
		let mut combinations_send: Vec<Vec<u32>> = Vec::new();

		if iter_global < SET_LENGTH {
			iter_global += 1;
			let combinations_received: Vec<Vec<u32>> = combinations_sets_inner(iter_global);
			// println!("\n====================Iter {iter_global} - NORMAL MODE=======================");
			
			for combination in combinations_received {
				for iter_local in 0..=2 {
					let mut combination_temp: Vec<u32> = combination.to_vec();
					combination_temp.push(iter_local);
					combinations_send.push(combination_temp);
				}	
			}
		} else {
			// println!("\n====================Iter {iter_global} - INIT MOD=========================");
			for iter_local in 0..=2 {
				let combination: Vec<u32> = vec![iter_local];
				combinations_send.push(combination);
			}
		}
		
		// print!("{:?} ", combinations_send);

		combinations_send
	}
	let combinations: Vec<Vec<u32>> = combinations_sets_inner(1);

	// Convert to Hint struct
	let combinations_hints: Vec<Hint> = Vec::new();
	for hint_vec in combinations {
		let mut hint_hint: Hint = Hint {
			exact: 0, 
			exist: 0, 
			null: 0
		};
		for value in hint_vec {
			match value {
				2 => hint_hint.exact += 1,
				1 => hint_hint.exist += 1,
				_ => hint_hint.null += 1
			}
		}
	}

	combinations_hints
}

/// Compute for each hint pattern its probability of happening given one set, by going trought all possible set combinations.
/// Input: set = Given set, set_combinations = vector of all possible set,
/// Returns: Vector of probabilities of each hints.
pub fn set_hint_probabilities(set_origin: &Vec<u32>, set_combinations: &Vec<Vec<u32>>, set_hint: &Vec<Hint>) -> Vec<WeightedHint> {
	let mut hint_quantities: Vec<WeightedHint> = Vec::new();
	// Go throught all sets and count 
	for set in set_combinations {
		// Compare
		let hint: Hint = similarities(&set_origin, &set);
		// Add to vector, check if already exists
		let mut found: bool = false;
		let mut i_wh: usize = 0;
		while i_wh < hint_quantities.len() && !found {
			found = hint_quantities[i_wh].hint == hint;
			i_wh += 1;
		}
		if found {
			hint_quantities[i_wh - 1].weight += 1;
		} else {
			hint_quantities.push(WeightedHint { 
				hint: hint,
				weight: 1 
			});
		}

	}

	hint_quantities
} 