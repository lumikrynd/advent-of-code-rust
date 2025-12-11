use std::cmp::Ordering::{Greater, Less};
use std::iter::once;

use super::Point;
use Turn::*;

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
	let right = turns.iter().filter(|t| matches!(t, Right(_))).count();
	let clockwise = right > (turns.len() / 2);

	return turns.into_iter().map(|t| map_turn(t, clockwise)).collect();

	fn map_turn(turn: Turn, clockwise: bool) -> Corner {
		match (clockwise, turn) {
			(true, Left(point)) => Inner(point),
			(true, Right(point)) => Outer(point),
			(false, Left(point)) => Outer(point),
			(false, Right(point)) => Inner(point),
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Corner {
	pub point: Point,
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
pub fn Outer(point: Point) -> Corner {
	Corner {
		point,
		corner_type: Type::Outer,
	}
}

#[allow(non_snake_case)]
pub fn Inner(point: Point) -> Corner {
	Corner {
		point,
		corner_type: Type::Inner,
	}
}

/// Makes 3 points into a corner. Assumes left = inner and right = outer
fn get_turn(points: (&Point, &Point, &Point)) -> Turn {
	let x_cmp = points.0.x.cmp(&points.2.x);
	let y_cmp = points.0.y.cmp(&points.2.y);

	// Assume going up or down
	let result = match (y_cmp, x_cmp) {
		(Greater, Less) => Left(points.1.clone()),
		(Greater, Greater) => Right(points.1.clone()),
		(Less, Less) => Right(points.1.clone()),
		(Less, Greater) => Left(points.1.clone()),
		_ => panic!("Same x or y???"),
	};

	// Invert if assumption is wrong
	if points.0.x == points.1.x {
		result
	} else {
		match result {
			Left(point) => Right(point),
			Right(point) => Left(point),
		}
	}
}

#[derive(Debug, PartialEq)]
enum Turn {
	Left(Point),
	Right(Point),
}

#[cfg(test)]
mod test {
	use super::*;

	fn square_case() -> ([Point; 4], [Corner; 4]) {
		let points = [
			Point(0, 0),
			Point(0, 2),
			Point(2, 2),
			Point(2, 0),
		];

		let expected = [
			Outer(Point(0, 0)),
			Outer(Point(0, 2)),
			Outer(Point(2, 2)),
			Outer(Point(2, 0)),
		];

		(points, expected)
	}

	#[test]
	fn square_clockwise() {
		let (points, expected) = square_case();
		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn square_counter_clockwise() {
		let (mut points, mut expected) = square_case();
		points.reverse();
		expected.reverse();

		assert_eq!(create_corners(&points), expected);
	}

	fn inner_case() -> ([Point; 6], [Corner; 6]) {
		let points = [
			Point(0, 0),
			Point(0, 2),
			Point(2, 2),
			Point(2, 1),
			Point(1, 1),
			Point(1, 0),
		];

		let expected = [
			Outer(Point(0, 0)),
			Outer(Point(0, 2)),
			Outer(Point(2, 2)),
			Outer(Point(2, 1)),
			Inner(Point(1, 1)),
			Outer(Point(1, 0)),
		];

		(points, expected)
	}

	#[test]
	fn with_inner() {
		let (points, expected) = inner_case();
		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn with_inner_counter_clockwise() {
		let (mut points, mut expected) = inner_case();
		points.reverse();
		expected.reverse();

		assert_eq!(create_corners(&points), expected);
	}

	#[test]
	fn get_turn_test() {
		let points = (&Point(2, 2), &Point(2, 4), &Point(1, 4));
		assert!(matches!(get_turn(points), Left(_)), "up left");

		let points = (&Point(2, 2), &Point(2, 4), &Point(3, 4));
		assert!(matches!(get_turn(points), Right(_)), "up right");

		let points = (&Point(2, 2), &Point(2, 0), &Point(1, 0));
		assert!(matches!(get_turn(points), Right(_)), "down right");

		let points = (&Point(2, 2), &Point(2, 0), &Point(3, 0));
		assert!(matches!(get_turn(points), Left(_)), "down left");

		let points = (&Point(2, 2), &Point(0, 2), &Point(0, 3));
		assert!(matches!(get_turn(points), Right(_)), "left up");

		let points = (&Point(2, 2), &Point(0, 2), &Point(0, 1));
		assert!(matches!(get_turn(points), Left(_)), "left down");

		let points = (&Point(2, 2), &Point(4, 2), &Point(4, 3));
		assert!(matches!(get_turn(points), Left(_)), "right up");

		let points = (&Point(2, 2), &Point(4, 2), &Point(4, 1));
		assert!(matches!(get_turn(points), Right(_)), "right down");
	}
}
