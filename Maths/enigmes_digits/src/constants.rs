// Chapter 3, "Chiffres"; ex. perso;
// "Digits of known constant".
// Find PI = 2 * PHI(n) / n**2
//		EULER_NUMBER = SUM(n=0; n=+1; 1 / n!)
// PRECISION is the 10 factor with wich float are converted to integers.

pub const PRECISION: i64 = 16; 
pub const ITERATIONS: usize = 32;

/// Convert usize to array of digits. From Shepmaster on stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
fn digits_with_precision(n_float: f64, precision: i64) -> Vec<u8> {
    let n: i64 = (n_float * (10.0f64.powf(precision as f64))) as i64;
	fn digits_inner(n: i64, xs: &mut Vec<u8>) {
        if n >= 10 {
            digits_inner(n / 10, xs);
        }
        xs.push((n % 10i64).try_into().unwrap());
    }
    let mut xs: Vec<u8> = Vec::new();
    digits_inner(n, &mut xs);
    xs
}

// Euler
/// Compute a factorial for n in N and n >= 0
pub fn factorial(n: i64) -> f64 {
	let mut result: f64 = 1.0;
	for factor in 1..=n {
		result *= factor as f64;
	}

	result
}

/// Compute the euler number and converts its PRECSIONth first digits it into a vector. 
pub fn approximation_euler_number(iterations: usize) -> Vec<u8> {
	let mut euler_number: f64 = 0.0;
	for n in 0..=iterations {
		euler_number += 1.0 / factorial(n as i64);
	}

	digits_with_precision(euler_number, PRECISION)
}

//Pi (with PHI)
/// PHI is the funtion that return the number of couple p and k > 0 that follow p ** 2 + k ** 2 = n ** 2
pub fn phi(n: u64) -> u64 {
	let mut count: u64 = 0;
	for p in 1..

}