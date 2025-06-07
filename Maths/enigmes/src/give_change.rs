// Hard problems: brute force and greedy algorithms
// Money and give changes

pub const SYSTEM_UE_LEN: usize = 9;
pub const SYSTEM_UE: [u32; SYSTEM_UE_LEN] = [1000, 500, 200, 100, 50, 20, 10, 5, 1];
pub const SYSTEM_PLT_LEN: usize = 11;
pub const SYSTEM_PLT: [u32; SYSTEM_PLT_LEN] = [29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1];

pub fn array_sum<const N: usize>(array: [u32; N]) -> u32 {
	let mut sum_number: u32 = 0;
	for n in array.iter() {
		sum_number += n;
	}

	sum_number
}

/// Return vector of the number of each container needed to divide the money sum. The monetary system should be ordered from greater to smaller. 
/// For optimal results, the system should be canonic
pub fn greedy<const N: usize>(mut return_money: u32, system: [u32; N]) -> [u32; N] {
	let mut system_uses: [u32; N] = [0; N];
	while return_money > 0u32 {
		let mut found: bool = false;
		let mut i: usize = 0;
		while i < N
			  &&
			  !found
		{
			if system[i] <= return_money {
				system_uses[i] += 1;
				return_money -= system[i];
				found = true;	
			}
			i += 1;
		}
	}

	system_uses
}

pub fn canonic_system<const N: usize>(system: [u32; N]) -> bool {
	let mut is_canonic = true;
	let mut i: usize = 0;
	while i < N - 2 {
		is_canonic = system[i] > system[i + 1] + system[i + 2];
		i += 1;
	}

	is_canonic
}