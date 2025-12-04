use std::iter::once;

use aoc_helpers::cartesian_set;

pub struct Map {
	height: usize,
	width: usize,
	state: Vec<Vec<bool>>,
}

impl Map {
	pub fn new(start: &Vec<Vec<bool>>) -> Self {
		let height = start.len();
		let width = start.first().map(Vec::len).unwrap_or(0);

		let middle = start
			.iter()
			.map(|line| add_padding(line.clone().into_iter(), false))
			.map(Iterator::collect);

		let state = add_padding(middle, vec![false; width + 2]);
		let state = state.collect();

		Self {
			width,
			height,
			state,
		}
	}

	pub fn count_accessible_rolls(&self) -> usize {
		cartesian_set(0..self.width, 0..self.height)
			.map(|cord| self.has_roll(cord) && self.is_accessible(cord))
			.filter(|moveable| *moveable)
			.count()
	}

	fn has_roll(&self, (x, y): (usize, usize)) -> bool {
		self.state[y + 1][x + 1]
	}

	fn is_accessible(&self, (x, y): (usize, usize)) -> bool {
		let count = cartesian_set(x..=x + 2, y..=y + 2)
			.map(|(x, y)| self.state[y][x])
			.filter(|roll| *roll)
			.count();

		count < 5 //less than 4 surrounding + self
	}
}

fn add_padding<T, I>(input: I, padding: T) -> impl Iterator<Item = T>
where
	T: Clone,
	I: Iterator<Item = T>,
{
	once(padding.clone())
		.chain(input)
		.chain(once(padding.clone()))
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::super::parse;
	use super::*;

	#[test]
	fn new_checking_dimensions() {
		let map = create_map_from(indoc! {"
		.@@@@@@@."});

		assert_eq!(map.height, 1);
		assert_eq!(map.width, 9);
		assert_eq!(map.state.len(), 3);

		for row in map.state {
			assert_eq!(row.len(), 11);
		}
	}

	#[test]
	fn count_accessible_rolls_none() {
		let map = create_map_from(indoc! {"
			.@@@@@@@.
			@@@@@@@@@
			@@@@@@@@@
			.@@@@@@@."});

		assert_eq!(map.count_accessible_rolls(), 0);
	}

	#[test]
	fn count_accessible_rolls_single() {
		let map = create_map_from(indoc! {"
			@@@@@@@@.
			@@@@@@@@@
			@@@@@@@@@
			.@@@@@@@."});

		assert_eq!(map.count_accessible_rolls(), 1);
	}

	fn create_map_from(input: &str) -> Map {
		let parsed = parse(input);
		Map::new(&parsed)
	}
}
