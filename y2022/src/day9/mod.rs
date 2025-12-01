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
		let moves = &self.moves;
		let count = count_tail_locations(moves, 1);
		Some(count.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let moves = &self.moves;
		let count = count_tail_locations(moves, 9);
		Some(count.to_string())
	}
}

fn count_tail_locations(moves: &Moves, rope_tail_length: usize) -> usize {
	let iter = moves.move_iter();

	let rope_states =
		iter.fold(vec![Rope::new(rope_tail_length)], |mut coll, dir| {
			let last = coll.last().unwrap();
			let next = last.move_direction(dir);
			coll.push(next);
			coll
		});

	let tail_locations: HashSet<Position> =
		rope_states.into_iter().map(|r| r.tail()).collect();

	tail_locations.iter().count()
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let moves = parsing::parse(input);
		let s = Solver { moves };
		Box::new(s)
	}
}

#[derive(Debug, PartialEq)]
struct Rope {
	head: Position,
	tail: Vec<Position>,
}

impl Rope {
	fn new(tail_len: usize) -> Rope {
		Rope {
			head: Position(0, 0),
			tail: vec![Position(0, 0); tail_len],
		}
	}

	fn move_direction(&self, dir: Direction) -> Self {
		let head = self.head.move_direction(dir);

		let mut prev = head;
		let tail = self.tail.iter().fold(vec![], move |mut acc, val| {
			let next = val.move_towards(prev);
			prev = next;
			acc.push(next);
			acc
		});
		Self { head, tail }
	}

	pub fn tail(&self) -> Position {
		match self.tail.last() {
			Some(t) => *t,
			None => self.head,
		}
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
	use super::parsing::parse;
	use super::*;

	use indoc::indoc;

	#[test]
	fn count_tail_locations_1() {
		let input = indoc! {"
			R 4
			U 4
			L 3
			D 1
			R 4
			D 1
			L 5
			R 2"};
		let moves = parse(input);

		let result = count_tail_locations(&moves, 1);
		assert_eq!(13, result);
	}

	#[test]
	fn count_tail_locations_2() {
		let input = indoc! {"
			R 5
			U 8
			L 8
			D 3
			R 17
			D 10
			L 25
			U 20"};
		let moves = parse(input);

		let result = count_tail_locations(&moves, 9);
		assert_eq!(36, result);
	}

	#[test]
	fn long_rope_move() {
		let mut rope = Rope::new(2);
		rope = rope.move_direction(Right);
		rope = rope.move_direction(Right);
		rope = rope.move_direction(Right);

		assert_eq!(
			rope,
			long_rope(vec![
				Position(3, 0),
				Position(2, 0),
				Position(1, 0)
			])
		);

		rope = rope.move_direction(Up);
		rope = rope.move_direction(Up);
		assert_eq!(
			rope,
			long_rope(vec![
				Position(3, 2),
				Position(3, 1),
				Position(2, 1)
			])
		);
	}

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
		let tail = vec![tail];
		Rope { head, tail }
	}

	fn long_rope(mut knots: Vec<Position>) -> Rope {
		let head = knots.remove(0);
		Rope { head, tail: knots }
	}
}
