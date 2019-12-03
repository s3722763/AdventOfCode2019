use std::fs::File;
use std::io::Read;
use std::str::FromStr;

/*Day 1*/
pub fn day1_first_run() -> Result<(), failure::Error> {
	if let Ok(data_lines_res) = get_day1_lines() {
		let data_lines = data_lines_res.split("\r\n");
		let mut sum = 0;

		for line in data_lines {
			let num = u32::from_str(line)?;
			//println!("{}", num);
			sum += day1_first_process(num);
		}

		println!("Day 1 part 2 result: {}", sum);
	}

	Ok(())
}

fn day1_first_process(mass: u32) -> u32 {
	(mass / 3) - 2
}

pub fn day1_second_run() -> Result<(), failure::Error> {
	if let Ok(data_lines_res) = get_day1_lines() {
		let data_lines = data_lines_res.split("\r\n");
		let mut sum: u32 = 0;

		for line in data_lines {
			let num = u32::from_str(line)?;
			sum += day1_second_process(num);
		}

		println!("Day 1 part 2 result: {}", sum);
	}

	Ok(())
}

fn day1_second_process(mut original_mass: u32) -> u32 {
	let mut fuel: u32 = 0;

	while (original_mass / 3) > 2 {
		let needed_fuel = day1_first_process(original_mass);
		fuel += needed_fuel;
		original_mass = needed_fuel;
	}

	fuel
}

fn get_day1_lines() -> Result<String, failure::Error> {
	let mut file = File::open("./day1.txt")?;
	let mut file_data = String::new();
	file.read_to_string(&mut file_data)?;

	Ok(file_data)
}

#[cfg(test)]
mod tests {
	use crate::day1::{day1_first_process, day1_second_process};

	#[test]
	fn day1_first_test() {
		assert_eq!(day1_first_process(12), 2);
		assert_eq!(day1_first_process(14), 2);
		assert_eq!(day1_first_process(1969), 654);
		assert_eq!(day1_first_process(100756), 33583);
	}

	#[test]
	fn day1_second_test() {
		assert_eq!(day1_second_process(12), 2);
		assert_eq!(day1_second_process(1969), 966);
		assert_eq!(day1_second_process(100756), 50346);
	}
}
