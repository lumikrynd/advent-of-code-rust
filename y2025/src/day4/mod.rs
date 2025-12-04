use aoc_helpers::PuzzleSolver;

mod map;
mod parsing;

use parsing::parse;

pub struct Solver {
	start_position: Vec<Vec<bool>>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		Box::new(Solver {
			start_position: parse(input),
		})
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let map = map::Map::new(&self.start_position);
		let count = map.count_accessible_rolls();
		Some(count.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn part_1() {
		let solver = Solver::new(EXAMPLE_INPUT);

		assert_eq!(solver.solve_part_1().unwrap(), "13");
	}

	const EXAMPLE_INPUT: &str = indoc! {"
		..@@.@@@@.
		@@@.@.@.@@
		@@@@@.@.@@
		@.@@@@..@.
		@@.@@@@.@@
		.@@@@@@@.@
		.@.@.@.@@@
		@.@@@.@@@@
		.@@@@@@@@.
		@.@.@@@.@."};
}
