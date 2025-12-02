use aoc_helpers::{PuzzleSolver, cartesian_set};

mod map;
use map::*;

pub struct Solver {
	map: Map,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let (x_size, y_size) = self.map.get_dimmensions();

		let visible = cartesian_set(0..x_size, 0..y_size)
			.filter(|(x, y)| is_vissible(&self.map, *x, *y))
			.count();

		Some(visible.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let (x_size, y_size) = self.map.get_dimmensions();

		let max_scenic_score = cartesian_set(0..x_size, 0..y_size)
			.map(|(x, y)| scenic_score(&self.map, x, y))
			.max()
			.unwrap_or(0);

		Some(max_scenic_score.to_string())
	}
}

fn scenic_score(map: &Map, x: usize, y: usize) -> usize {
	let (x_size, y_size) = map.get_dimmensions();
	let height = &map.get(x, y);

	let count_east = (0..x)
		.rev()
		.take_while(delay_1(move |v| &map.get(*v, y) < height))
		.count();
	let count_west = ((x + 1)..x_size)
		.take_while(delay_1(move |v| &map.get(*v, y) < height))
		.count();
	let count_north = (0..y)
		.rev()
		.take_while(delay_1(move |v| &map.get(x, *v) < height))
		.count();
	let count_south = ((y + 1)..y_size)
		.take_while(delay_1(move |v| &map.get(x, *v) < height))
		.count();

	count_east * count_west * count_north * count_south
}

fn delay_1(pred: impl Fn(&usize) -> bool) -> impl FnMut(&usize) -> bool {
	let mut run = true;
	move |v| {
		let ret = run;
		run = pred(v);
		ret
	}
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let s = Solver {
			map: Map::new(input),
		};
		Box::new(s)
	}
}

fn is_vissible(map: &Map, x: usize, y: usize) -> bool {
	let (x_size, y_size) = map.get_dimmensions();

	if 0 == x || 0 == y {
		return true;
	}

	let height: i16 = map.get(x, y).into();

	height > max_in_area(map, 0, x - 1, y, y)
		|| height > max_in_area(map, x + 1, x_size - 1, y, y)
		|| height > max_in_area(map, x, x, 0, y - 1)
		|| height > max_in_area(map, x, x, y + 1, y_size - 1)
}

// xs for x_start, xe for end
fn max_in_area(map: &Map, xs: usize, xe: usize, ys: usize, ye: usize) -> i16 {
	cartesian_set(xs..=xe, ys..=ye)
		.map(|(x, y)| map.get(x, y).into())
		.max()
		.unwrap_or(-1)
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn solve_part_1() {
		let input = indoc! {"
			30373
			25512
			65332
			33549
			35390"};

		let solver = Solver::new(input);

		assert_eq!("21", solver.solve_part_1().unwrap().as_str());
	}

	#[test]
	fn solve_part_2() {
		let input = indoc! {"
			30373
			25512
			65332
			33549
			35390"};

		let solver = Solver::new(input);

		assert_eq!("8", solver.solve_part_2().unwrap().as_str());
	}

	#[test]
	fn scenic_score_test() {
		let input = indoc! {"
			9999999
			9999999
			9991999
			9912199
			9991999
			9999999
			9999999"};

		let map = Map::new(input);

		assert_eq!(0, scenic_score(&map, 0, 0), "Edge should result in 0");
		assert_eq!(4, scenic_score(&map, 1, 3), "East count doesn't work");
		assert_eq!(4, scenic_score(&map, 5, 3), "West count doesn't work");
		assert_eq!(4, scenic_score(&map, 3, 1), "South count doesn't work");
		assert_eq!(4, scenic_score(&map, 3, 5), "North count doesn't work");
	}

	#[test]
	fn is_vissible_false() {
		let input = indoc! {"
			030
			435
			090"};

		let map = Map::new(input);

		assert!(!is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_west() {
		let input = indoc! {"
			939
			235
			999"};

		let map = Map::new(input);

		assert!(is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_east() {
		let input = indoc! {"
			939
			432
			999"};

		let map = Map::new(input);

		assert!(is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_north() {
		let input = indoc! {"
			929
			433
			999"};

		let map = Map::new(input);

		assert!(is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_south() {
		let input = indoc! {"
			949
			433
			909"};

		let map = Map::new(input);

		assert!(is_vissible(&map, 1, 1));
	}
}
