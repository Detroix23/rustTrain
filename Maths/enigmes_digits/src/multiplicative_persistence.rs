// Chapter 3, "Chiffres"; ex.2p.61;
// "Multiplicative persistence".
// Ex: 673 -> 6 * 7 * 3 = 126 -> 1 * 2 * 6 = 12 -> 1 * 2 = 2

pub fn digit_multiplication(mut number: u32) -> u32 {
	let mut multiplication: u32 = number % 10;
	number /= 10;
	while number != 0 {
		multiplication *= number % 10;
		number /= 10;
	}

	multiplication
}

pub fn persistence(mut number: u32) -> u32 {
	let mut count_persistence: u32 = 0;
	while number / 10 > 0 {
		number = digit_multiplication(number);
		count_persistence += 1;

		// println!("- {}", number);
	}

	count_persistence
}

pub fn max_persistence(limit: u32) -> (u32, u32) {
	let mut n: u32 = 1;
	let mut max: u32 = 0;
	let mut max_numbers: Vec<u32> = Vec::new();
	// Search all persistences
	while n < limit {
		let n_persistence = persistence(n);
		if n_persistence > max {
			max = n_persistence;
			max_numbers = Vec::new();
		} else if n_persistence == max {
			max_numbers.push(n);
		}
		n += 1;
	}
	// Find min of max
	let min_max: u32 = *max_numbers.iter().min().unwrap();

	(max, min_max)
}