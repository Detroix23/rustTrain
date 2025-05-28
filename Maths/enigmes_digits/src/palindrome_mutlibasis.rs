// Chapter 3, "Chiffres"; ex.7p.63;
// "Palindrome in mutliple basis" (modif. from "Twice palindrome").
// Find a number that is a palindrome in multiple basis.
// Ex: 45 = 101101 => palindrome in binary


pub fn vector_digits(mut number: u32) -> Vec<u8> {
	let mut digits: Vec<u8> = Vec::new();
	while number > 0 {
		let digit = (number % 10) as u8;
		number /= 10;
		digits.push(digit);
	}

	digits
}

pub fn number_palindrome(number: u32) -> bool {
	let digits: Vec<u8> = vector_digits(number);
	let length: usize = digits.len();
	let mut i: usize = 0;

	let mut is_palindrome: bool = true;
	while is_palindrome && i < (length as f64 / 2f64).floor() as usize {
		is_palindrome = digits[i] == digits[length - i - 1usize];
		i += 1;
	}

	is_palindrome
}

