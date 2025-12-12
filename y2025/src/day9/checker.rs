use aoc_helpers::combinations_set;
use std::cmp::Ordering::{Greater, Less};
use std::iter::once;
use std::ops::Not;

use super::Point;

pub fn create_checker(boxes: &[Point]) -> AreaChecker {
	//scan_for_potential_problems(boxes);

	let corners = create_corners(boxes);
	let lines = create_lines(&corners);
	AreaChecker::new(lines)
}

fn create_lines(corners: &[Corner]) -> Vec<(Point, Point)> {
	let len = corners.len();

	(0..(len - 1))
		.map(|i| (&corners[i], &corners[i + 1]))
		.chain(once((&corners[len - 1], &corners[0])))
		.map(|(a, b)| create_line(*a, *b))
		.collect()
}

fn create_line(start: Corner, end: Corner) -> (Point, Point) {
	let invert = start.point > end.point;
	let horizontal = is_horizontal(&start.point, &end.point);

	let s_diff = find_difference(start, invert, horizontal, false);
	let s = start.point;
	let s = Point(s.x + s_diff.0, s.y + s_diff.1);

	let e_diff = find_difference(end, invert, horizontal, true);
	let e = end.point;
	let e = Point(e.x + e_diff.0, e.y + e_diff.1);

	if s < e { (s, e) } else { (e, s) }
}

fn find_difference(
	corner: Corner,
	invert: bool,
	horizontal: bool,
	end: bool,
) -> (i64, i64) {
	// Assume line going up, start point, right turn
	let mut diff = match corner.corner_type {
		Type::Outer => (-1, -1),
		Type::Inner => (1, 1),
	};
	if end {
		diff = rotate(diff);
	}
	if horizontal {
		diff = rotate(diff)
	}
	if invert {
		diff = (-diff.0, -diff.1);
	}
	if matches!(corner.direction, Direction::Left) {
		diff = match horizontal {
			true => (diff.0, -diff.1),
			false => (-diff.0, diff.1),
		}
	}
	diff
}

/// Rotate vector clockwise
fn rotate((x, y): (i64, i64)) -> (i64, i64) {
	(y, -x)
}

/*
Possible fun optimization which isn't needed.
Instead of storing a vector of the lines and always have to traverse them all
we could instead have a BTreeMap where (for the horizontal) the key would be x
and the value would be a vector of pointers to all the horizontal lines which
exist for that x-value. A new key would be needed for every x-value where a
line starts or ends. The value would be the same as the previous except the
line which is added or removed.
With this we could easily find all active lines for a given point in x, and
afterwards see if our vertical line crosses any of them.

In practice it seems like a lot more lines would have had to be defined before
it is even needed.
 */
pub struct AreaChecker {
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

	pub fn check(&self, (a, b): (Point, Point)) -> bool {
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

fn create_corners(boxes: &[Point]) -> Vec<Corner> {
	let middle = (0..(boxes.len() - 2))
		.map(|i| (&boxes[i], &boxes[i + 1], &boxes[i + 2]));

	let len = boxes.len();
	let first = (&boxes[len - 1], &boxes[0], &boxes[1]);
	let last = (&boxes[len - 2], &boxes[len - 1], &boxes[0]);
	let all = once(first).chain(middle).chain(once(last));

	let turns: Vec<_> = all.map(get_turn).collect();

	map_turns(turns)
}

fn map_turns(turns: Vec<Turn>) -> Vec<Corner> {
	let right = turns
		.iter()
		.filter(|t| t.direction == Direction::Right)
		.count();
	let clockwise = right > (turns.len() / 2);

	return turns.into_iter().map(|t| map_turn(t, clockwise)).collect();

	fn map_turn(turn: Turn, clockwise: bool) -> Corner {
		match (clockwise, turn.direction) {
			(true, Direction::Left) => Inner(turn.point, Direction::Left),
			(true, Direction::Right) => Outer(turn.point, Direction::Right),
			(false, Direction::Left) => Outer(turn.point, Direction::Left),
			(false, Direction::Right) => Inner(turn.point, Direction::Right),
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Corner {
	pub point: Point,
	pub direction: Direction,
	pub corner_type: Type,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Type {
	/// A corner folding around the inside of the model
	/// Like the 4 corners of a square
	Outer,

	/// A corner digging into the inside of the model
	Inner,
}

#[allow(non_snake_case)]
fn Outer(point: Point, direction: Direction) -> Corner {
	Corner {
		point,
		direction,
		corner_type: Type::Outer,
	}
}

#[allow(non_snake_case)]
fn Inner(point: Point, direction: Direction) -> Corner {
	Corner {
		point,
		direction,
		corner_type: Type::Inner,
	}
}

/// Makes 3 points into a corner. Assumes left = inner and right = outer
fn get_turn(points: (&Point, &Point, &Point)) -> Turn {
	let x_cmp = points.0.x.cmp(&points.2.x);
	let y_cmp = points.0.y.cmp(&points.2.y);
	let point = *points.1;

	// Assume going up or down
	let direction = match (y_cmp, x_cmp) {
		(Greater, Less) => Direction::Left,
		(Greater, Greater) => Direction::Right,
		(Less, Less) => Direction::Right,
		(Less, Greater) => Direction::Left,
		_ => panic!("Same x or y???"),
	};

	// Invert if assumption is wrong
	if points.0.x == points.1.x {
		Turn { direction, point }
	} else {
		match direction {
			Direction::Left => Right(point),
			Direction::Right => Left(point),
		}
	}
}

#[derive(Debug, PartialEq)]
struct Turn {
	point: Point,
	direction: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
	Left,
	Right,
}

#[allow(non_snake_case)]
fn Left(point: Point) -> Turn {
	Turn {
		point,
		direction: Direction::Left,
	}
}

#[allow(non_snake_case)]
fn Right(point: Point) -> Turn {
	Turn {
		point,
		direction: Direction::Right,
	}
}

#[cfg(test)]
mod test {
	use super::Direction::Left;
	use super::Direction::Right;
	use super::*;

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
			Outer((1, 1), Left),
			Outer((3, 1), Left),
			Outer((3, 3), Left),
			Outer((2, 3), Left),
			Inner((2, 2), Right),
			Outer((1, 2), Left),
		];

		let result = create_lines(&input);

		let expected = vec![
			(Point(0, 0), Point(4, 0)),
			(Point(4, 0), Point(4, 4)),
			(Point(1, 4), Point(4, 4)),
			(Point(1, 3), Point(1, 4)),
			(Point(0, 3), Point(1, 3)),
			(Point(0, 0), Point(0, 3)),
		];

		assert_eq!(result, expected);
	}

	#[test]
	fn create_line_test() {
		let result = create_line(Outer((2, 2), Right), Outer((2, 8), Right));
		assert_eq!(result, (Point(1, 1), Point(1, 9)), "Up, Outer");

		let result = create_line(Outer((2, 2), Right), Outer((5, 2), Right));
		assert_eq!(result, (Point(1, 3), Point(6, 3)), "Right, Outer");

		let result = create_line(Inner((2, 2), Right), Inner((2, 8), Right));
		assert_eq!(result, (Point(3, 3), Point(3, 7)), "Up, Inner");

		let result = create_line(Inner((2, 2), Right), Inner((5, 2), Right));
		assert_eq!(result, (Point(3, 1), Point(4, 1)), "Right, Inner");

		// Normalize
		let result = create_line(Outer((3, 1), Right), Outer((1, 1), Right));
		assert_eq!(result, (Point(0, 0), Point(4, 0)), "normalize hor");

		let result = create_line(Outer((1, 3), Right), Outer((1, 1), Right));
		assert_eq!(result, (Point(2, 0), Point(2, 4)), "normalize vert");

		// Left
		let result = create_line(Outer((2, 2), Left), Outer((2, 8), Left));
		assert_eq!(result, (Point(3, 1), Point(3, 9)), "Up, Outer, Left");

		let result = create_line(Outer((2, 2), Left), Outer((5, 2), Left));
		assert_eq!(result, (Point(1, 1), Point(6, 1)), "Right, Outer, Left");

		let result = create_line(Inner((2, 2), Left), Inner((2, 8), Left));
		assert_eq!(result, (Point(1, 3), Point(1, 7)), "Up, Inner, Left");

		let result = create_line(Inner((2, 2), Left), Inner((5, 2), Left));
		assert_eq!(result, (Point(3, 3), Point(4, 3)), "Right, Inner, Left");
	}

	fn square_case() -> [Point; 4] {
		[
			Point(0, 0),
			Point(0, 2),
			Point(2, 2),
			Point(2, 0),
		]
	}

	#[test]
	fn square_clockwise() {
		let points = square_case();
		let expected = [
			Outer((0, 0), Right),
			Outer((0, 2), Right),
			Outer((2, 2), Right),
			Outer((2, 0), Right),
		];

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn square_counter_clockwise() {
		let mut points = square_case();
		points.reverse();
		let expected = [
			Outer((2, 0), Left),
			Outer((2, 2), Left),
			Outer((0, 2), Left),
			Outer((0, 0), Left),
		];

		assert_eq!(create_corners(&points), expected);
	}

	fn inner_case() -> [Point; 6] {
		[
			Point(0, 0),
			Point(0, 2),
			Point(2, 2),
			Point(2, 1),
			Point(1, 1),
			Point(1, 0),
		]
	}

	#[test]
	fn with_inner() {
		let points = inner_case();
		let expected = [
			Outer((0, 0), Right),
			Outer((0, 2), Right),
			Outer((2, 2), Right),
			Outer((2, 1), Right),
			Inner((1, 1), Left),
			Outer((1, 0), Right),
		];

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn with_inner_counter_clockwise() {
		let mut points = inner_case();
		points.reverse();

		let expected = [
			Outer((1, 0), Left),
			Inner((1, 1), Right),
			Outer((2, 1), Left),
			Outer((2, 2), Left),
			Outer((0, 2), Left),
			Outer((0, 0), Left),
		];

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn get_turn_test() {
		let points = (&Point(2, 2), &Point(2, 4), &Point(1, 4));
		assert!(matches!(get_turn(points).direction, Left), "up left");

		let points = (&Point(2, 2), &Point(2, 4), &Point(3, 4));
		assert!(matches!(get_turn(points).direction, Right), "up right");

		let points = (&Point(2, 2), &Point(2, 0), &Point(1, 0));
		assert!(matches!(get_turn(points).direction, Right), "down right");

		let points = (&Point(2, 2), &Point(2, 0), &Point(3, 0));
		assert!(matches!(get_turn(points).direction, Left), "down left");

		let points = (&Point(2, 2), &Point(0, 2), &Point(0, 3));
		assert!(matches!(get_turn(points).direction, Right), "left up");

		let points = (&Point(2, 2), &Point(0, 2), &Point(0, 1));
		assert!(matches!(get_turn(points).direction, Left), "left down");

		let points = (&Point(2, 2), &Point(4, 2), &Point(4, 3));
		assert!(matches!(get_turn(points).direction, Left), "right up");

		let points = (&Point(2, 2), &Point(4, 2), &Point(4, 1));
		assert!(matches!(get_turn(points).direction, Right), "right down");
	}

	#[allow(non_snake_case)]
	fn Outer((x, y): (i64, i64), direction: Direction) -> Corner {
		super::Outer(Point(x, y), direction)
	}

	#[allow(non_snake_case)]
	fn Inner((x, y): (i64, i64), direction: Direction) -> Corner {
		super::Inner(Point(x, y), direction)
	}
}
