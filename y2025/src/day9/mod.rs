use std::{
	collections::BinaryHeap,
	iter::once,
	ops::{Add, Not, Sub},
	process::Output,
};

use aoc_helpers::{PuzzleSolver, combinations_set};

use corners::*;
mod corners;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
	x: i64,
	y: i64,
}

#[allow(non_snake_case)]
fn Point(x: i64, y: i64) -> Point {
	Point { x, y }
}

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
		let result = find_biggest_tiled_area(&self.boxes);
		Some(result.to_string())
	}
}

fn find_biggest_area(boxes: &[Point]) -> i64 {
	let mut set: BinaryHeap<_> = combinations_set(boxes.iter())
		.map(|(a, b)| HeapItem::from(*a, *b))
		.collect();

	set.pop().unwrap().0
}

fn find_biggest_tiled_area(boxes: &[Point]) -> i64 {
	//scan_for_potential_problems(boxes);

	let checker = create_checker(boxes);

	let mut set: BinaryHeap<_> = combinations_set(boxes.iter())
		.map(|(a, b)| HeapItem::from(*a, *b))
		.collect();

	loop {
		let value = set.pop().unwrap();

		if checker.check(value.1) {
			break value.0;
		}
	}
}

fn create_checker(boxes: &[Point]) -> AreaChecker {
	let corners = create_corners(boxes);
	let lines = create_lines(&corners);
	AreaChecker::new(lines)
}

fn create_lines(corners: &Vec<Corner>) -> Vec<(Point, Point)> {
	let len = corners.len();

	(0..(len - 1))
		.map(|i| (&corners[i], &corners[i + 1]))
		.chain(once((&corners[len - 1], &corners[0])))
		.map(|(a, b)| create_line(*a, *b))
		.collect()
}

fn create_line(start: Corner, end: Corner) -> (Point, Point) {
	let (start, end) = if start.point > end.point {
		(end, start)
	} else {
		(start, end)
	};

	// Inner is made shorter because it shouldn't block
	// It doesn't block because each direction is either an edge or the inner model.
	let horizontal = is_horizontal(&start.point, &end.point);
	let s = start.point;
	let s = match (start.corner_type, horizontal) {
		(Type::Outer, _) => start.point,
		(Type::Inner, true) => Point(s.x + 1, s.y),
		(Type::Inner, false) => Point(s.x, s.y + 1),
	};

	let e = end.point;
	let e = match (end.corner_type, horizontal) {
		(Type::Outer, _) => end.point,
		(Type::Inner, true) => Point(e.x - 1, e.y),
		(Type::Inner, false) => Point(e.x, e.y - 1),
	};

	(s, e)
}

struct AreaChecker {
	horizontal: Vec<(Point, Point)>,
	vertical: Vec<(Point, Point)>,
}

impl AreaChecker {
	fn new(lines: Vec<(Point, Point)>) -> AreaChecker {
		let (horizontal, vertical) = lines
			.into_iter()
			.map(|(a, b)| if a < b { (a, b) } else { (b, a) })
			.partition(|(a, b)| is_horizontal(a, b));

		AreaChecker {
			horizontal,
			vertical,
		}
	}

	fn check(&self, (a, b): (Point, Point)) -> bool {
		let (x_min, x_max) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
		let (y_min, y_max) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

		self.check_vertical_line(x_min, y_min, y_max)
			&& self.check_vertical_line(x_max, y_min, y_max)
			&& self.check_horizontal_line(y_max, x_min, x_max)
			&& self.check_horizontal_line(y_min, x_min, x_max)
	}

	fn check_vertical_line(&self, x: i64, y_min: i64, y_max: i64) -> bool {
		self.horizontal
			.iter()
			.any(|(a, b)| between(x, a.x, b.x) && between(a.y, y_min, y_max))
			.not()
	}

	fn check_horizontal_line(&self, y: i64, x_min: i64, x_max: i64) -> bool {
		self.vertical
			.iter()
			.any(|(a, b)| between(y, a.y, b.y) && between(a.x, x_min, x_max))
			.not()
	}
}

fn between(v: i64, min: i64, max: i64) -> bool {
	min <= v && v <= max
}

fn is_horizontal(a: &Point, b: &Point) -> bool {
	a.y == b.y
}

/// Used to scan for patterns of edges being so close together that it would be valid to draw a
/// rectangle across them. Just did a visual check lines one x or y coordinate apart wasn't
/// actually close to each other.
#[allow(dead_code)]
fn scan_for_potential_problems(boxes: &[Point]) {
	for pair in combinations_set(boxes.iter()) {
		if pair.0.x.abs_diff(pair.1.x) == 1 {
			println!("To close x values: {:?}", pair);
		}
		if pair.0.y.abs_diff(pair.1.y) == 1 {
			println!("To close y values: {:?}", pair);
		}
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct HeapItem(i64, (Point, Point));

impl HeapItem {
	fn from(a: Point, b: Point) -> Self {
		HeapItem(area(&a, &b), (a, b))
	}
}

fn area(a: &Point, b: &Point) -> i64 {
	((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
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
	use super::*;
	use indoc::indoc;

	#[test]
	fn part_1_test() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "50");
	}

	#[test]
	fn part_2_test() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_2().unwrap(), "24");
	}

	#[test]
	fn area_checker_test() {
		let checker = AreaChecker::new(vec![
			(Point(10, 10), Point(20, 10)),
			(Point(30, 5), Point(30, 15)),
		]);
		assert!(checker.check((Point(1, 9), Point(5, 5))));

		assert!(!checker.check((Point(3, 5), Point(15, 12))));
		assert!(!checker.check((Point(15, 12), Point(3, 5))), "inverted");
		assert!(!checker.check((Point(15, 9), Point(20, 11))));

		assert!(!checker.check((Point(20, 5), Point(40, 2))));
		assert!(!checker.check((Point(20, 20), Point(40, 14))));
	}

	#[test]
	fn create_lines_test() {
		let input = vec![
			Outer(1, 1),
			Outer(3, 1),
			Outer(3, 3),
			Outer(2, 3),
			Inner(2, 2),
			Outer(1, 2),
		];

		let result = create_lines(&input);

		let expected = vec![
			(Point(1, 1), Point(3, 1)),
			(Point(3, 1), Point(3, 3)),
			(Point(2, 3), Point(3, 3)),
			(Point(2, 3), Point(2, 3)),
			(Point(1, 2), Point(1, 2)),
			(Point(1, 1), Point(1, 2)),
		];

		assert_eq!(result, expected);
	}

	#[test]
	fn create_line_test() {
		let result = create_line(Outer(1, 1), Outer(3, 1));
		assert_eq!(result, (Point(1, 1), Point(3, 1)), "easy case");

		let result = create_line(Outer(1, 1), Outer(1, 3));
		assert_eq!(result, (Point(1, 1), Point(1, 3)), "vertical");

		// Normalize
		let result = create_line(Outer(3, 1), Outer(1, 1));
		assert_eq!(result, (Point(1, 1), Point(3, 1)), "normalize hor");

		let result = create_line(Outer(1, 3), Outer(1, 1));
		assert_eq!(result, (Point(1, 1), Point(1, 3)), "normalize vert");

		// Expand
		let result = create_line(Inner(1, 1), Inner(5, 1));
		assert_eq!(result, (Point(2, 1), Point(4, 1)), "shrink hor");

		let result = create_line(Inner(1, 1), Inner(1, 5));
		assert_eq!(result, (Point(1, 2), Point(1, 4)), "shrink vert");
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

	#[allow(non_snake_case)]
	fn Outer(x: i64, y: i64) -> Corner {
		corners::Outer(Point(x, y))
	}

	#[allow(non_snake_case)]
	fn Inner(x: i64, y: i64) -> Corner {
		corners::Inner(Point(x, y))
	}
}
