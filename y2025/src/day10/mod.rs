use aoc_helpers::PuzzleSolver;
use std::iter::{from_fn, once};

use parsing::parse;
mod parsing;

pub struct Solver {
	machines: Vec<Machine>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = Solver {
			machines: parse(input),
		};
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let sum: u32 = self
			.machines
			.iter()
			.map(find_fewest_presses)
			.map(u32::from)
			.sum();

		Some(sum.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

#[derive(Debug, PartialEq)]
struct Machine {
	lights: Vec<Light>,
	buttons: Vec<Button>,
}

#[derive(Debug, PartialEq)]
struct Light {
	should_be_on: bool,
	joltage: Joltage,
}

#[derive(Debug, PartialEq)]
struct Button {
	lights: Vec<LightIndex>,
}

type LightIndex = usize;
type Joltage = u32;

fn find_fewest_presses(machine: &Machine) -> u8 {
	let goal: Vec<_> = machine.lights.iter().map(|l| l.should_be_on).collect();

	let res = generate_combinations(&machine.buttons)
		.filter(|comb| is_valid_combination(comb, &goal))
		.map(|comb| comb.len())
		.min()
		.unwrap_or(0);

	res as u8
}

fn is_valid_combination(combination: &[&Button], goal: &[bool]) -> bool {
	let mut result = vec![false; goal.len()];

	for i in combination.iter().flat_map(|b| b.lights.iter()) {
		result[*i] = !result[*i];
	}

	result == goal
}

fn generate_combinations(
	buttons: &[Button],
) -> impl Iterator<Item = Vec<&Button>> {
	combinations_iterator(buttons.len()).map(move |combs| {
		buttons
			.iter()
			.zip(combs)
			.filter_map(|(v, b)| if b { Some(v) } else { None })
			.collect()
	})
}

fn combinations_iterator(size: usize) -> impl Iterator<Item = Vec<bool>> {
	let mut value = vec![false; size];
	let next_value = move || {
		for i in 0..size {
			if value[i] {
				value[i] = false;
			} else {
				value[i] = true;
				return Some(value.clone());
			}
		}
		None
	};

	once(vec![false; size]).chain(from_fn(next_value))
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn part_1_test() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "7");
	}

	#[test]
	fn find_fewest_presses_test() {
		let machine = Machine(
			vec![
				Light(false, 3),
				Light(true, 5),
				Light(true, 4),
				Light(false, 7),
			],
			vec![
				Button(vec![3]),
				Button(vec![1, 3]),
				Button(vec![2]),
				Button(vec![2, 3]),
				Button(vec![0, 2]),
				Button(vec![0, 1]),
			],
		);

		assert_eq!(find_fewest_presses(&machine), 2);
	}

	#[test]
	fn is_valid_combination_test() {
		let goal = [false, false, false];
		assert_eq!(is_valid_combination(&[], &goal), true, "empty true");

		let goal = [false, true, false];
		assert_eq!(is_valid_combination(&[], &goal), false, "empty false");

		let goal = [false, true, false];
		let buttons = [&Button(vec![1])];
		assert_eq!(is_valid_combination(&buttons, &goal), true, "Single press");

		let goal = [false, true, false];
		let buttons = [&Button(vec![2]), &Button(vec![1, 2])];
		assert_eq!(is_valid_combination(&buttons, &goal), true, "Single press");
	}

	#[test]
	fn combinations_iterator_test() {
		let mut it = combinations_iterator(3);
		assert_eq!(it.next(), Some(vec![false, false, false]));
		assert_eq!(it.next(), Some(vec![true, false, false]));
		assert_eq!(it.next(), Some(vec![false, true, false]));
		assert_eq!(it.next(), Some(vec![true, true, false]));
		assert_eq!(it.next(), Some(vec![false, false, true]));
		assert_eq!(it.next(), Some(vec![true, false, true]));
		assert_eq!(it.next(), Some(vec![false, true, true]));
		assert_eq!(it.next(), Some(vec![true, true, true]));
		assert_eq!(it.next(), None);
	}

	#[allow(non_snake_case)]
	fn Machine(lights: Vec<Light>, buttons: Vec<Button>) -> Machine {
		Machine { lights, buttons }
	}

	#[allow(non_snake_case)]
	fn Light(should_be_on: bool, joltage: Joltage) -> Light {
		Light {
			should_be_on,
			joltage,
		}
	}

	#[allow(non_snake_case)]
	fn Button(lights: Vec<LightIndex>) -> Button {
		Button { lights }
	}

	const EXAMPLE: &str = indoc! {"
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
		[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"};
}
