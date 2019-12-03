use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn day2_first_run() -> Result<(), failure::Error> {
	let mut file = File::open("./day2.txt")?;
	let mut file_data = String::new();
	file.read_to_string(&mut file_data)?;
	let data_str_split = file_data.split(",");
	let mut memory: Vec<u32> = Vec::new();

	for data in data_str_split {
		let number = u32::from_str(data)?;
		memory.push(number);
	}

	memory[1] = 12;
	memory[2] = 2;

	day2_first_process(&mut memory);
	println!("Day 2 part 1 result: {}", memory[0]);

	Ok(())
}

fn day2_first_process(memory: &mut Vec<u32>) -> Result<(), failure::Error> {
	let mut pc = 0;

	'running: loop{
		let opcode = memory[pc];
		if cfg!(test) {
			println!("Memory: {:?}\nInstruction: {:?}", memory, opcode);
		}

		if opcode == 1 || opcode == 2 {
			run_op_code(&pc, opcode, memory);
		} else if opcode == 99 {
			break 'running;
		} else {
			println!("Invalid code: {}", opcode);
		}

		pc += 4;
	}

	Ok(())
}

fn run_op_code(pc: &usize, opcode: u32, memory: &mut Vec<u32>) {
	let pos_1 = memory[pc+1];
	let pos_2 = memory[pc+2];
	let pos_res = memory[pc+3];

	let first_no = memory[pos_1 as usize];
	let second_no = memory[pos_2 as usize];

	match opcode {
		1 => {
			let res = first_no + second_no;
			memory[pos_res as usize] = res;
		},
		2 => {
			let res = first_no * second_no;
			memory[pos_res as usize] = res;
		},
		_ => {}
	}
}

#[cfg(test)]
mod tests {
	use crate::day2::day2_first_process;

	#[test]
	fn day2_first_test() {
		let mut memory = vec!(1,0,0,0,99);
		day2_first_process(&mut memory);
		assert_eq!(memory, vec!(2,0,0,0,99));

		let mut memory = vec!(2,3,0,3,99);
		day2_first_process(&mut memory);
		assert_eq!(memory, vec!(2,3,0,6,99));

		let mut memory = vec!(2,4,4,5,99,0);
		day2_first_process(&mut memory);
		assert_eq!(memory, vec!(2,4,4,5,99,9801));

		let mut memory = vec!(1,1,1,4,99,5,6,0,99);
		day2_first_process(&mut memory);
		assert_eq!(memory, vec!(30,1,1,4,2,5,6,0,99));
	}
}