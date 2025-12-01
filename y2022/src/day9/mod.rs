use std::collections::HashSet;

use aoc_helpers::PuzzleSolver;
use moves::*;

mod moves;
mod parsing;

pub struct Solver {
	moves: Moves,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let iter = self.moves.move_iter();

		let fun_times = iter.fold(vec![Rope::new()], |mut coll, dir| {
			let last = coll.last().unwrap();
			let next = last.move_direction(dir);
			coll.push(next);
			coll
		});

		let tail_locations: HashSet<_> =
			fun_times.into_iter().map(|r| r.tail).collect();
		let count = tail_locations.len();

		Some(count.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let moves = parsing::parse(input);
		let moves = Moves(moves);
		let s = Solver { moves };
		Box::new(s)
	}
}

#[derive(Debug, PartialEq)]
struct Rope {
	head: Position,
	tail: Position,
}

impl Rope {
	fn new() -> Rope {
		Rope {
			head: Position(0, 0),
			tail: Position(0, 0),
		}
	}

	fn move_direction(&self, dir: Direction) -> Self {
		let head = self.head.move_direction(dir);
		let tail = self.tail.move_towards(head);
		Self { head, tail }
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

impl Position {
	fn move_direction(&self, dir: Direction) -> Self {
		match dir {
			Direction::Up => Position(self.0, self.1 + 1),
			Direction::Down => Position(self.0, self.1 - 1),
			Direction::Left => Position(self.0 - 1, self.1),
			Direction::Right => Position(self.0 + 1, self.1),
		}
	}

	fn move_towards(&self, other: Position) -> Position {
		if self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1 {
			return *self;
		}

		let c_0 = (other.0 - self.0).signum();
		let c_1 = (other.1 - self.1).signum();

		Position(self.0 + c_0, self.1 + c_1)
	}
}

#[cfg(test)]
mod test {
	use super::Direction::*;
	use super::*;

	#[test]
	fn rope_move() {
		let init = rope(Position(1, 1), Position(0, 0));
		assert_eq!(
			init.move_direction(Up),
			rope(Position(1, 2), Position(1, 1)),
			"Up",
		);
		assert_eq!(
			init.move_direction(Left),
			rope(Position(0, 1), Position(0, 0)),
			"Left"
		);
		assert_eq!(
			init.move_direction(Down),
			rope(Position(1, 0), Position(0, 0)),
			"Down"
		);
		assert_eq!(
			init.move_direction(Right).move_direction(Right),
			rope(Position(3, 1), Position(2, 1)),
			"Down"
		);
	}

	#[test]
	fn position_move() {
		let pos = Position(3, 5);
		assert_eq!(pos.move_direction(Up), Position(3, 6), "up");
		assert_eq!(pos.move_direction(Down), Position(3, 4), "down");
		assert_eq!(pos.move_direction(Left), Position(2, 5), "left");
		assert_eq!(pos.move_direction(Right), Position(4, 5), "right");
	}

	#[test]
	fn position_towards() {}

	fn rope(head: Position, tail: Position) -> Rope {
		Rope { head, tail }
	}
}
