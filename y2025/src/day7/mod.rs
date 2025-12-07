use std::collections::HashMap;

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
		let res = find_timeline_count(self.start, &self.splitters);
		Some(res.to_string())
	}
}

fn find_split_count(from: Coord, splitters: &[Coord]) -> usize {
	let mut calc = BeamCalculator::new(splitters);
	calc.traverse(from);
	calc.visited.len()
}

fn find_timeline_count(from: Coord, splitters: &[Coord]) -> usize {
	let mut calc = BeamCalculator::new(splitters);
	calc.traverse(from)
}

struct BeamCalculator {
	splitters: Vec<Coord>,
	visited: HashMap<Coord, usize>,
}

impl BeamCalculator {
	pub fn new(splitters: &[Coord]) -> Self {
		assert!(splitters.is_sorted_by_key(|f| f.y()));
		Self {
			splitters: splitters.to_owned(),
			visited: HashMap::new(),
		}
	}

	/// returns alternative timelines
	pub fn traverse(&mut self, from: Coord) -> usize {
		let split = self
			.splitters
			.iter()
			.filter(|f| f.x() == from.x())
			.find(|f| f.y() > from.y());

		match split {
			None => 1,
			Some(c) => {
				if let Some(existing) = self.visited.get(c) {
					return *existing;
				}

				let c = *c;
				let (x, y) = (c.x(), c.y());
				let res = self.traverse(Coord::new(x + 1, y))
					+ self.traverse(Coord::new(x - 1, y));

				self.visited.insert(c, res);
				res
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
	fn part_2() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_2().unwrap(), "40")
	}

	#[test]
	fn find_split_empty() {
		let result = find_split_count(Coord::new(0, 0), &[]);
		assert_eq!(result, 0);
	}

	#[test]
	fn find_split_single() {
		let result =
			find_split_count(Coord::new(1, 0), &[Coord::new(1, 10)]);
		assert_eq!(result, 1);
	}

	#[test]
	fn find_split_ignore_above() {
		let result =
			find_split_count(Coord::new(1, 14), &[Coord::new(1, 10)]);
		assert_eq!(result, 0);
	}

	#[test]
	fn find_split_merge() {
		let result = find_split_count(
			Coord::new(5, 0),
			&[
				Coord::new(5, 2),
				Coord::new(4, 4),
				Coord::new(6, 4),
				Coord::new(5, 6),
			],
		);
		assert_eq!(result, 4);
	}

	#[test]
	fn find_timelines_empty() {
		let result = find_timeline_count(Coord::new(0, 0), &[]);
		assert_eq!(result, 1);
	}

	#[test]
	fn find_timelines_single() {
		let result =
			find_timeline_count(Coord::new(1, 0), &[Coord::new(1, 10)]);
		assert_eq!(result, 2);
	}

	#[test]
	fn find_timelines_merge() {
		let result = find_timeline_count(
			Coord::new(5, 0),
			&[
				Coord::new(5, 2),
				Coord::new(4, 4),
				Coord::new(6, 4),
				Coord::new(5, 6),
			],
		);
		assert_eq!(result, 6);
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
