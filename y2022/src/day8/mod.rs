use aoc_helpers::PuzzleSolver;

mod map;
use map::*;

pub struct Solver {
	map: Map,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let (x_size, y_size) = self.map.get_dimmensions();

		let mut visible = 0;
		for x in 0..x_size {
			for y in 0..y_size {
				if is_vissible(&self.map, x, y) {
					visible += 1;
				}
			}
		}

		Some(visible.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
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
	(xs..=xe)
		.flat_map(move |x| (ys..=ye).map(move |y| (x, y)))
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
			35390
		"};

		let map = Solver::new(input);

		assert_eq!("21", map.solve_part_1().unwrap().as_str());
	}

	#[test]
	fn is_vissible_false() {
		let input = indoc! {"
			030
			435
			090
		"};

		let map = Map::new(input);

		assert_eq!(false, is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_west() {
		let input = indoc! {"
			939
			235
			999
		"};

		let map = Map::new(input);

		assert_eq!(true, is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_east() {
		let input = indoc! {"
			939
			432
			999
		"};

		let map = Map::new(input);

		assert_eq!(true, is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_north() {
		let input = indoc! {"
			929
			433
			999
		"};

		let map = Map::new(input);

		assert_eq!(true, is_vissible(&map, 1, 1));
	}

	#[test]
	fn is_vissible_from_south() {
		let input = indoc! {"
			949
			433
			909
		"};

		let map = Map::new(input);

		assert_eq!(true, is_vissible(&map, 1, 1));
	}
}
