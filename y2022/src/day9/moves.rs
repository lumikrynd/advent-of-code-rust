use aoc_helpers::pub_wrapper;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub_wrapper!(Moves, Vec<Move>);

impl Moves {
	pub fn move_iter(&self) -> MovesIter<'_> {
		MovesIter::new(self.iter())
	}
}

pub struct MovesIter<'a> {
	inner: std::slice::Iter<'a, Move>,
	current: Option<Direction>,
	count: usize,
}

impl<'a> MovesIter<'a> {
	pub fn new(inner: std::slice::Iter<'a, Move>) -> MovesIter<'a> {
		MovesIter {
			inner,
			current: None,
			count: 0,
		}
	}
}

impl<'a> Iterator for MovesIter<'a> {
	type Item = Direction;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count == 0 {
			let Move { direction, steps } = self.inner.next()?;
			self.current = Some(*direction);
			self.count = *steps;
		}

		self.count -= 1;
		self.current
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
	direction: Direction,
	steps: usize,
}

impl Move {
	pub fn new(direction: Direction, steps: usize) -> Move {
		if steps < 1 {
			panic!("Steps can't be less than 1")
		}
		Move { direction, steps }
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn move_iterator() {
		let input = Moves(vec![
			Move::new(Direction::Up, 3),
			Move::new(Direction::Right, 2),
		]);

		let mut iter = input.move_iter();
		assert_eq!(Direction::Up, iter.next().unwrap());
		assert_eq!(Direction::Up, iter.next().unwrap());
		assert_eq!(Direction::Up, iter.next().unwrap());
		assert_eq!(Direction::Right, iter.next().unwrap());
		assert_eq!(Direction::Right, iter.next().unwrap());
		assert_eq!(None, iter.next());
		assert_eq!(None, iter.next());
	}
}
