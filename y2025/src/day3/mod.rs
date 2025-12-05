use aoc_helpers::PuzzleSolver;

type Digit = u8;
type Voltage = u64;

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
		let result = fun_name(&self.batteries, 2);
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let result = fun_name(&self.batteries, 12);
		Some(result.to_string())
	}
}

fn fun_name(batteries: &[Vec<Digit>], count: usize) -> Voltage {
	batteries
		.iter()
		.map(|battery| best_battery_config(battery, count) as Voltage)
		.sum()
}

fn best_battery_config(battery: &[Digit], count: usize) -> Voltage {
	let len = battery.len();
	let mut start = 0;

	let mut result = 0;
	for i in (0..count).rev() {
		let end = len - i;
		let area = &battery[start..end];

		let digit = area.iter().max().unwrap();
		start = start + area.iter().position(|x| x == digit).unwrap() + 1;

		result += (*digit as Voltage) * Voltage::pow(10, i as u32);
	}

	result
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
		assert_eq!(best_battery_config(&[2, 9], 2), 29);
		assert_eq!(best_battery_config(&[9, 5, 8, 3, 6], 2), 98);
		assert_eq!(best_battery_config(&[3, 5, 8, 3, 6, 9], 2), 89);

		assert_eq!(best_battery_config(&[3, 5, 8, 3, 6, 9], 1), 9);

		assert_eq!(best_battery_config(&[3, 5, 8, 3, 6, 9], 3), 869);
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

	#[test]
	fn part_2_test() {
		let solver = Solver::new(EXAMPLE_INPUT);

		let result = solver.solve_part_2();

		assert_eq!(result, Some("3121910778619".to_string()))
	}

	const EXAMPLE_INPUT: &str = "\
		987654321111111\n\
		811111111111119\n\
		234234234234278\n\
		818181911112111";
}
