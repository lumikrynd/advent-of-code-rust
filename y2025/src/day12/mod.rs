use aoc_helpers::PuzzleSolver;

mod parsing;

#[derive(Debug, PartialEq)]
pub struct Solver {
	shapes: Vec<Shape>,
	regions: Vec<RegionSpec>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = parsing::parse(input);
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
pub struct Shape {
	fields: Vec<Vec<bool>>
}

#[derive(Debug, PartialEq)]
pub struct RegionSpec {
	dimensions: (u8, u8),
	counts: Vec<usize>,
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn part_1() {
		let _solver = Solver::new(EXAMPLE);
	}

	const EXAMPLE: &str = indoc! {"
		0:
		###
		##.
		##.

		1:
		###
		##.
		.##

		2:
		.##
		###
		##.

		3:
		##.
		###
		##.

		4:
		###
		#..
		###

		5:
		###
		.#.
		###

		4x4: 0 0 0 0 2 0
		12x5: 1 0 1 0 2 2
		12x5: 1 0 1 0 3 2"};
}
