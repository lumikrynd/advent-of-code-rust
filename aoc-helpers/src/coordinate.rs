use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate<T>
where
	T: Clone + Copy + Debug + Display + PartialEq + Eq + Hash,
{
	x: T,
	y: T,
}

impl<T> Coordinate<T>
where
	T: Clone + Copy + Debug + Display + PartialEq + Eq + Hash,
{
	pub fn new(x: T, y: T) -> Coordinate<T> {
		Coordinate { x, y }
	}

	pub fn x(&self) -> T {
		self.x
	}

	pub fn y(&self) -> T {
		self.y
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn can_be_used_as_key() {
		let mut map = HashSet::new();
		assert!(map.insert(new(1, 2)));
		assert!(!map.insert(new(1, 2)));
		assert!(map.insert(new(2, 2)));
		assert!(map.insert(new(1, 3)));
		assert!(!map.insert(new(2, 2)));
	}

	fn new(x: usize, y: usize) -> Coordinate<usize> {
		Coordinate::new(x, y)
	}
}

