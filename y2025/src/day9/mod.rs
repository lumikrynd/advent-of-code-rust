use std::collections::BinaryHeap;

use aoc_helpers::{PuzzleSolver, combinations_set};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point(i64, i64);

pub struct Solver {
	boxes: Vec<Point>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = Solver {
			boxes: parse(input),
		};
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let result = find_biggest_area(&self.boxes);
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn find_biggest_area(boxes: &[Point]) -> i64 {
	let mut set : BinaryHeap<_> = combinations_set(boxes.iter())
		.map(|(a, b)| HeapItem::from(*a, *b))
		.collect();

	set.pop().unwrap().0
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct HeapItem(i64, (Point, Point));

impl HeapItem {
	fn from(a: Point, b: Point) -> Self {
		HeapItem(area(&a, &b), (a, b))
	}
}

fn area(a: &Point, b: &Point) -> i64 {
	((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn parse(input: &str) -> Vec<Point> {
	input
		.lines()
		.map(|l| l.split_once(',').unwrap())
		.map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
		.map(|(x, y)| Point(x, y))
		.collect()
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn part_1_test() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "50");
	}

	#[test]
	fn parse_test() {
		let input = "7,1\n11,1\n11,7";
		let result = parse(input);
		assert_eq!(result, vec![Point(7, 1), Point(11, 1), Point(11, 7)])
	}

	#[test]
	fn area_test() {
		let a = Point(11, 1);
		let b = Point(7, 3);

		assert_eq!(area(&a, &b), 15);
	}

	const EXAMPLE: &str = indoc! {"
		7,1
		11,1
		11,7
		9,7
		9,5
		2,5
		2,3
		7,3"};
}
