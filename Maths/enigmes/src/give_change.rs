// Hard problems: brute force and greedy algorithms
// Money and give changes

// Define systems. Extra 1s at the end are for uniformity, for having each system the same length.
pub const SYSTEM_LEN: usize = 11;
pub const SYSTEM_UE: [u32; SYSTEM_LEN] = [1000, 500, 200, 100, 50, 20, 10, 5, 2, 1, 1];
pub const SYSTEM_PLT: [u32; SYSTEM_LEN] = [29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1];
pub const SYSTEM_BI: [u32; SYSTEM_LEN] = [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];

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

pub fn systems_efficency<const N: usize>(systems: [[u32; SYSTEM_LEN]; N], max_return: usize) -> [u32; N] {
	let mut score: [u32; N] = [0; N];
	let mut container_sum: u32 = 0;
	for to_return in 1..max_return {
		for system_id in 0..N {
			container_sum = array_sum(greedy(to_return as u32, systems[system_id]));
			score[system_id] += container_sum;
		}
	}
	score
}