// MASTER MIND
// Bot search.



// Import from Main.
use crate::{POOL_SIZE, SET_LENGTH, HashMap};
// Import from Checks.
use crate::{checks::{similarities, Hint}};

/// Structure that stores infos on next probabilities and bits
pub struct SetScore {
	pub probability: f64,
	pub bits: f64
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
			//println!("\n====================Iter {iter_global} - NORMAL MODE=======================");
			
			for combination in combinations_received {
				for iter_local in 0..=2 {
					let mut combination_temp: Vec<u32> = combination.to_vec();
					combination_temp.push(iter_local);
					combinations_send.push(combination_temp);
				}	
			}
		} else {
			//println!("\n====================Iter {iter_global} - INIT MOD=========================");
			for iter_local in 0..=2 {
				let combination: Vec<u32> = vec![iter_local];
				combinations_send.push(combination);
			}
		}
		
		//print!("{:?} ", combinations_send);

		combinations_send
	}
	let combinations: Vec<Vec<u32>> = combinations_sets_inner(1);

	// Convert to Hint struct
	let mut combinations_hints: Vec<Hint> = Vec::new();
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

		if !combinations_hints.contains(&hint_hint) {
			combinations_hints.push(hint_hint);
		}
	}

	combinations_hints
}

/// Find, for a given set, the number by set of hint-set that would correspond 
/// Input: set = Given set, set_combinations = vector of all possible set,
/// Returns: HashMap of the quantity of each hint-set.
pub fn set_hint_quantities(set_origin: &Vec<u32>, set_combinations: &Vec<Vec<u32>>, set_hints: &Vec<Hint>) -> HashMap<Hint, u32> {
	// Init all hints
	let mut hint_quantities: HashMap<Hint, u32> = HashMap::new();
	for set_hint in set_hints {
		hint_quantities.insert(set_hint.clone(), 0u32);
	}
	// Go throught all sets
	for set in set_combinations {
		// Compare
		let hint: Hint = similarities(&set_origin, &set);
		// Add to map
		hint_quantities.insert(hint.clone(), hint_quantities[&hint] + 1);
	}

	hint_quantities
}

/// Compute for each hint-set its probability of happening and and bits of a set given its quantity map, by going trought all possible set combinations.
pub fn set_hint_score(hint_quantities: HashMap<Hint, u32>) -> HashMap<Hint, SetScore> {
	let mut hint_probabilities: HashMap<Hint, SetScore> = HashMap::new();
	// Sum.
	let mut total_sum: u32 = 0;
	for (_, quantity) in &hint_quantities {
		total_sum += quantity;
	}
	//println!("sum: {}", total_sum);
	// Proba.
	for (hint, quantity) in hint_quantities {
		let probability: f64 = quantity as f64 / total_sum as f64;
		let mut bits: f64 = 0.0_f64;
		if probability != 0.0_f64 {
			bits = -probability.log2();
		} else {
			bits = 0.0_f64;
		}
		let score: SetScore = SetScore {
			probability,
			bits
		};
		hint_probabilities.insert(hint, score);
	}

	hint_probabilities
}

/// Compute the entropy of a set, given its hint score. Allow a general score.
pub fn set_hint_score_entropy(hints_score: HashMap<Hint, SetScore>) -> f64 {
	let mut entropy: f64 = 0.0f64;
	for (_, scores) in hints_score {
		entropy += scores.probability * scores.bits;
	}

	entropy
}

/// Comprehensive function to compute entropy of a given set.
pub fn set_entropy(set: &Vec<u32>, combinations_sets: &Vec<Vec<u32>>, combinations_hint: &Vec<Hint>) -> f64 {
	// Hint quantities.
    let hint_quantities: HashMap<Hint, u32> = set_hint_quantities(&set, &combinations_sets, &combinations_hint);
    /*
	println!("Hint quantities:");
    for (hint, weight) in &hint_quantities {
        println!("- q = {} -> {}{} {}{} {}{}", weight, hint.exact, "AC", hint.exist, "IS", hint.null, "NO");
    }
	*/
    // println!("q: {}", _hint_quantities.len());
    // Hint probability.
    let hint_score: HashMap<Hint, SetScore> = set_hint_score(hint_quantities);
    /*
	println!("Hint probability:");
	for (hint, score) in &hint_score {
        println!("- p = {:.2}%; b = {:.2}bits -> {}{} {}{} {}{}", score.probability * 100f64, score.bits,hint.exact, CORRECT_PLACEMENT, hint.exist, CORRECT_VALUE, hint.null, CORRECT_NON);
    }
	*/
    // Set overhaul score.
    let entropy = set_hint_score_entropy(hint_score);
	/*
	println!("Entropy:");
    println!("- E = {:.5}bits", {entropy});
	 */
	entropy
}

/// Compute entropy of all possible combinations given and returns a sorted HashMap.
pub fn all_set_entropy(combinations_sets: &Vec<Vec<u32>>, combinations_hint: &Vec<Hint>) -> HashMap<Vec<u32>, f64> {
	let mut entropies: HashMap<Vec<u32>, f64> = HashMap::new();
	for set in combinations_sets {
		entropies.insert(set.clone(), set_entropy(set, combinations_sets, combinations_hint));
	}

	entropies
}