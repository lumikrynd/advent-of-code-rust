use aoc_helpers::{PuzzleSolver, errors::AocError, wrapper};

mod parsing;

pub struct Solver(Vec<CliLine>);

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		None
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

#[derive(Debug, PartialEq)]
enum CliLine {
	Output(LsOutput),
	Command(Command),
}

#[derive(Debug, PartialEq)]
enum LsOutput {
	Dir { name: String },
	File { name: String, size: usize },
}

#[derive(Debug, PartialEq)]
enum Command {
	Cd(Cd),
	Ls,
}

#[derive(Debug, PartialEq)]
enum Cd {
	Root,
	Parent,
	Dir { name: String },
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let solver = parsing::parse(input);
		Box::new(solver)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::error::Error;

	#[test]
	fn just_a_test() -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
