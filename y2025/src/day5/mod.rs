use std::ops::RangeInclusive;

use aoc_helpers::PuzzleSolver;
use range_reduction::reduce;

mod parsing;
mod range_reduction;

type Id = usize;
type Range = RangeInclusive<Id>;

#[derive(Debug, PartialEq)]
pub struct Solver {
	fresh: Vec<Range>,
	ingredients: Vec<Id>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = parsing::parse(input);
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let fresh_count = self
			.ingredients
			.iter()
			.filter(|i| self.fresh.iter().any(|f| f.contains(i)))
			.count();

		Some(fresh_count.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let reduced = reduce(&self.fresh);
		let count: usize =
			reduced.iter().map(|f| f.end() - f.start() + 1).sum();

		Some(count.to_string())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn part_1() {
		let solver = Solver::new(EXAMPE_INPUT);
		assert_eq!(solver.solve_part_1().unwrap(), "3");
	}

	#[test]
	fn part_2() {
		let solver = Solver::new(EXAMPE_INPUT);
		assert_eq!(solver.solve_part_2().unwrap(), "14");
	}

	const EXAMPE_INPUT: &str = "\
		3-5\n10-14\n16-20\n12-18\n\n\
		1\n5\n8\n11\n17\n32";
}
