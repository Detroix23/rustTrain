
// Rules
#![allow(unused_doc_comments)]
#![allow(unused_variables)]


fn main() {
    
	// I - Some vars
	// a) Shadowing
	let a: u32 = 15;
	println!("# Shadowing");
	println!("Var a={a} (u32)");
	let a: isize = -15;
	println!("Var a={a} (isize)");
	

	// b) Data types
	println!("# Data type");

	let exemple_integer: i32 = 256;
	println!("- Integer = {exemple_integer}");

	let exemple_float: f32 = 15.051;
	println!("- Float = {exemple_float}");

	let addition_crosstype: f32 = exemple_float + exemple_integer as f32;
	println!("- Crosstype addition: {exemple_integer} + {exemple_float} = {addition_crosstype} (using 'as')");
	
	let exemple_tuple: (f32, f32, f32) = (156.15, 3215.2, 321.0);
	let (exemple_tuple_x, exemple_tuple_y, _exemple_tuple_z) = exemple_tuple;
	let exemple_tuple_z: f32 = exemple_tuple.2;
	println!("- Tuple: (unpacking the tuple) x = {exemple_tuple_x}, y = {exemple_tuple_y}, (accessing by index) z = {exemple_tuple_z}");

	let exemple_array: [u32; 2] = [651, 0x1A];
	println!("- Array: (using array indexing) [0]={0}, [1]={1}", exemple_array[0], exemple_array[1]);

	let exemple_grid: [[i32; 10]; 10] = [[0; 10]; 10];
	println!("- Grid array (2D [[i32; 10]; 10]): ");
	for line in exemple_grid.iter() {
		for tile in line.iter() {
			print!("{tile}");
		}
		println!();
	}

	let exemple_basic_string1: &str = "Hello slice str!";
	let exemple_basic_string2: String = String::from("Hello String!");
	let exemple_basic_string3: String = "Hello method 'to_string'".to_string();
	let exemple_basic_string4: String = "Hello method 'to_owned'".to_owned();
	let exemple_basic_string5: &str = &exemple_basic_string4[..];
	const exemple_const_string: &str = "Hello const; must use slice str.";
	println!("Strings: ({exemple_basic_string1}), ({exemple_basic_string2}), ({exemple_const_string}), ({exemple_basic_string5})");

	


}


