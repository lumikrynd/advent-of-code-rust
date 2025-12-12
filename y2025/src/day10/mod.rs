use aoc_helpers::PuzzleSolver;

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
		None
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
