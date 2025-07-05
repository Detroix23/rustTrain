// Chapter 3, "Chiffres"; ex.13p.65;
// The "exponant at the end".
// This problem aim to find which a^n and with the digits n.
// E.g: 2^36 = 68_719_476_736.
// a, n ∈ R+

pub fn check_end_digits(a: u128, n: u32) -> bool {
	let n_digits = digits(n.try_into().unwrap());
	let a_digits = digits(a);

	print!("e:{}, {:?}", a, a_digits);

	let mut matching: bool = a_digits.len() >= n_digits.len();
	let mut digit_pointer: usize = 1;
	
	while matching && digit_pointer <= n_digits.len() {
		if a_digits[a_digits.len() - digit_pointer] != n_digits[n_digits.len() - digit_pointer] {
			matching = false;
		}
		digit_pointer += 1;
	}

	matching
}

/// Convert usize to array of digits. From Shepmaster on stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
fn digits(n: u128) -> Vec<usize> {
    fn digits_inner(n: u128, xs: &mut Vec<usize>) {
        if n >= 10 {
            digits_inner(n / 10, xs);
        }
        xs.push((n % 10u128).try_into().unwrap());
    }
    let mut xs = Vec::new();
    digits_inner(n, &mut xs);
    xs
}

/// Iterate to find the number of solutions required. 0 to have no restrictions on iterations.
pub fn try_for_a(a: u128, solutions_required: usize, max_iterations: usize) -> Vec<usize> {
	let mut iteration: usize = 1;
	let mut solutions: Vec<usize> = Vec::new();

	print!("Try to find exponent: ");

	while solutions.len() < solutions_required 
	      && 
		  ((max_iterations > 0 && iteration < max_iterations) || max_iterations == 0) {
		if check_end_digits(a.pow(iteration.try_into().unwrap()), iteration.try_into().unwrap()) {
			solutions.push(iteration);
		}
		
		print!("{}, ", iteration);
		
		iteration += 1;
	}

	solutions
}

