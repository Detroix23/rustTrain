// MASTER MIND
// Check and computing side

use regex::Regex;

pub fn convert_input_set(set_string: String) -> Vec<u32> {
	let regex_cut_commas: Regex = Regex::new(r"[0-9]+").unwrap();
	let set: Vec<u32> = regex_cut_commas.find_iter(&set_string).map(|m| m.as_str().parse::<u32>().unwrap()).collect();

	set
}

/// Compute and compare 2 sets and tells how many exact (2) and correct values (1) there are; or nothing (0).
/// The sets must be of same size.
pub fn similarities(set_base: Vec<u32>, set_comparison: Vec<u32>) -> Vec<u8> {
	let mut results: Vec<u8> = Vec::new();
	let mut confounded_values: Vec<u32> = Vec::new();
	let mut i: usize = 0;
	while i < set_comparison.len() {
		// Exact (2).
		if set_base[i] == set_comparison[i] {
			results.push(2u8);
		// Existing value (1), check already found
		} else if confounded_values.contains(&set_comparison[i]) {
			results.push(1u8);
		// Existing value (1).
		} else {
			let mut value_exists: bool = false;
			let mut j: usize = 0;
			while j < set_base.len() && !value_exists {
				value_exists = set_comparison[i] == set_base[j];
				j += 1;
			}
			if value_exists {
				confounded_values.push(set_comparison[i]);
				results.push(1u8);
			} else {
				results.push(0u8);
			}
		}
		i += 1;
	}

	results.sort_by(|a, b| b.cmp(a));
	results
}