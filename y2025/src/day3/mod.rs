use aoc_helpers::PuzzleSolver;

type Digit = u8;

pub struct Solver {
	batteries: Vec<Vec<Digit>>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let batteries = parse(input);
		Box::new(Solver { batteries })
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let result: u32 = self
			.batteries
			.iter()
			.map(|battery| best_battery_config(battery) as u32)
			.sum();

		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn best_battery_config(battery: &[Digit]) -> u8 {
	let len = battery.len();
	let first_digit = battery[..(len - 1)].iter().max().unwrap();
	let first_index = battery.iter().position(|x| x == first_digit).unwrap();
	let second_digit = battery[(first_index+1)..].iter().max().unwrap();
	first_digit * 10 + second_digit
}

fn parse(input: &str) -> Vec<Vec<Digit>> {
	fn parse_line(input: &str) -> Vec<Digit> {
		input.chars().map(parse_char).collect()
	}

	fn parse_char(c: char) -> Digit {
		c.to_digit(10).unwrap() as Digit
	}

	input.lines().map(parse_line).collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn best_battery_config_test() {
		assert_eq!(best_battery_config(&[2, 9]), 29);
		assert_eq!(best_battery_config(&[9, 5, 8, 3, 6]), 98);
		assert_eq!(best_battery_config(&[3, 5, 8, 3, 6, 9]), 89);
	}

	#[test]
	fn parse_test() {
		let input = "";
		let result = parse(input);
		let expect: [[Digit; 0]; 0] = [];
		assert_eq!(result, expect, "Empty");

		let input = "1234";
		let result = parse(input);
		assert_eq!(result, [[1, 2, 3, 4]], "Single line");

		let input = "12345\n67890";
		let result = parse(input);
		assert_eq!(result, [[1, 2, 3, 4, 5], [6, 7, 8, 9, 0]], "Single line");
	}

	#[test]
	fn part_1_test() {
		let solver = Solver::new(EXAMPLE_INPUT);

		let result = solver.solve_part_1();

		assert_eq!(result, Some("357".to_string()))
	}

	const EXAMPLE_INPUT: &str = "\
		987654321111111\n\
		811111111111119\n\
		234234234234278\n\
		818181911112111";
}
