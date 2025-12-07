use std::collections::HashSet;

use aoc_helpers::PuzzleSolver;

type Coord = aoc_helpers::Coordinate<usize>;

mod parsing;

#[derive(Debug, PartialEq)]
pub struct Solver {
	start: Coord,
	splitters: Vec<Coord>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = parsing::parse(input);
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let res = find_split_count(self.start, &self.splitters);
		Some(res.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn find_split_count(from: Coord, splitters: &Vec<Coord>) -> usize {
	let mut calc = BeamCalculator::new(splitters);
	calc.find_split_count(from)
}

struct BeamCalculator {
	splitters: Vec<Coord>,
	visited: HashSet<Coord>,
}

impl BeamCalculator {
	pub fn new(splitters: &Vec<Coord>) -> Self {
		assert!(splitters.is_sorted_by_key(|f| f.y()));
		Self {
			splitters: splitters.clone(),
			visited: HashSet::new(),
		}
	}

	pub fn find_split_count(&mut self, from: Coord) -> usize {
		let split = self
			.splitters
			.iter()
			.filter(|f| f.x() == from.x())
			.find(|f| f.y() > from.y());

		match split {
			None => 0,
			Some(c) => {
				let new = self.visited.insert(*c);
				if !new {
					return 0;
				}

				let (x, y) = (c.x(), c.y());
				1 + self.find_split_count(Coord::new(x + 1, y))
					+ self.find_split_count(Coord::new(x - 1, y))
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn part_1() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "21")
	}

	#[test]
	fn find_split_empty() {
		let result = find_split_count(Coord::new(0, 0), &vec![]);
		assert_eq!(result, 0);
	}

	#[test]
	fn find_split_single() {
		let result =
			find_split_count(Coord::new(1, 0), &vec![Coord::new(1, 10)]);
		assert_eq!(result, 1);
	}

	#[test]
	fn find_split_ignore_above() {
		let result =
			find_split_count(Coord::new(1, 14), &vec![Coord::new(1, 10)]);
		assert_eq!(result, 0);
	}

	#[test]
	fn find_split_merge() {
		let result = find_split_count(
			Coord::new(5, 0),
			&vec![
				Coord::new(5, 2),
				Coord::new(4, 4),
				Coord::new(6, 4),
				Coord::new(5, 6),
			],
		);
		assert_eq!(result, 4);
	}

	const EXAMPLE: &str = indoc! {"
		.......S.......
		...............
		.......^.......
		...............
		......^.^......
		...............
		.....^.^.^.....
		...............
		....^.^...^....
		...............
		...^.^...^.^...
		...............
		..^...^.....^..
		...............
		.^.^.^.^.^...^.
		..............."};
}
