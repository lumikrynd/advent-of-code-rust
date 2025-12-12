use std::cmp::Ordering::{Greater, Less};
use std::iter::once;

use super::Point;

pub fn create_corners(boxes: &[Point]) -> Vec<Corner> {
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
pub struct Corner {
	pub point: Point,
	pub direction: Direction,
	pub corner_type: Type,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
	/// A corner folding around the inside of the model
	/// Like the 4 corners of a square
	Outer,

	/// A corner digging into the inside of the model
	Inner,
}

#[allow(non_snake_case)]
pub fn Outer(point: Point, direction: Direction) -> Corner {
	Corner {
		point,
		direction,
		corner_type: Type::Outer,
	}
}

#[allow(non_snake_case)]
pub fn Inner(point: Point, direction: Direction) -> Corner {
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
	let point = points.1.clone();

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
pub enum Direction {
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
			Outer(Point(0, 0), Right),
			Outer(Point(0, 2), Right),
			Outer(Point(2, 2), Right),
			Outer(Point(2, 0), Right),
		];

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn square_counter_clockwise() {
		let mut points = square_case();
		points.reverse();
		let expected = [
			Outer(Point(2, 0), Left),
			Outer(Point(2, 2), Left),
			Outer(Point(0, 2), Left),
			Outer(Point(0, 0), Left),
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
			Outer(Point(0, 0), Right),
			Outer(Point(0, 2), Right),
			Outer(Point(2, 2), Right),
			Outer(Point(2, 1), Right),
			Inner(Point(1, 1), Left),
			Outer(Point(1, 0), Right),
		];

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn with_inner_counter_clockwise() {
		let mut points = inner_case();
		points.reverse();

		let expected = [
			Outer(Point(1, 0), Left),
			Inner(Point(1, 1), Right),
			Outer(Point(2, 1), Left),
			Outer(Point(2, 2), Left),
			Outer(Point(0, 2), Left),
			Outer(Point(0, 0), Left),
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
}
