// Chapter 3, "Chiffres"; ex.1p.61;
// The "[pages of the] dictionnary".
// A number of digit is given and the program ought to find how much pages you can index with them.
// E.g: 23 digits = 1 * 9 + 2 * 7 => 16 pages = 9 + 7.

/// Find the number of page created by a given number of digits
pub fn count_pages(digits: u32) -> u32 {
	let mut pages: u32 = 0;
	let mut pages_digits: u32 = 0;

	while pages_digits < digits {
		pages += 1;
		pages_digits += (pages as f64).log10().floor() as u32 + 1u32;
		
		//println!("p{}, d{}", pages, pages_digits);
	}


	pages
}


/// Find the number of digits required for a given number of pages (reciprocal of the problem). Use logarithm10 to find the number of digits.
pub fn count_digits(pages: u32) -> u32 {
	let mut digits: u32 = 0; 
	for page in 1..=pages {
		digits += ((page as f32).log10().floor()) as u32 + 1u32;
	}

	digits
}
