use aoc_helpers::PuzzleSolver;

mod parsing;

pub struct Solver<'a> {
	cli_lines: Vec<CliLine<'a>>,
}

impl<'a> PuzzleSolver for Solver<'a> {
	fn solve_part_1(&self) -> Option<String> {
		None
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

impl<'a> Solver<'a> {
	pub fn new(input: &'a str) -> Box<Solver<'a>> {
		let s = Solver {
			cli_lines: parsing::parse(input),
		};
		Box::new(s)
	}
}

#[derive(Debug, PartialEq)]
enum CliLine<'a> {
	Output(LsOutput<'a>),
	Command(Command<'a>),
}

#[derive(Debug, PartialEq)]
enum LsOutput<'a> {
	Dir { name: &'a str },
	File { name: &'a str, size: usize },
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
	Cd(Cd<'a>),
	Ls,
}

#[derive(Debug, PartialEq)]
enum Cd<'a> {
	Root,
	Parent,
	Dir { name: &'a str },
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
