use aoc_helpers::Point2D;
use std::iter::once;
use super::*;

pub fn parse(input: &str) -> Solver {
	let mut lines = input.lines();
	let start = parse_start(lines.next().unwrap());
	let splitters = parse_splitters(lines);
	Solver { start, splitters }
}

fn parse_start(first_line: &str) -> aoc_helpers::Point2D<usize> {
	let start_x = first_line.find('S').unwrap();
	Coord::new(start_x, 0)
}

fn parse_splitters<'l>(lines: impl Iterator<Item = &'l str>) -> Vec<Coord> {
	once("")
		.chain(lines)
		.enumerate()
		.flat_map(|(y, line)| {
			line.chars().enumerate().filter_map(move |(x, c)| {
				if c == '^' {
					Some(Point2D::new(x, y))
				} else {
					None
				}
			})
		})
		.collect()
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn parse_test() {
		let input = indoc! {"
			....S...
			........
			^...^...
			........
			...^.^.."};

		let result = parsing::parse(input);

		let start = Coord::new(4, 0);
		let splitters = vec![
			Coord::new(0, 2),
			Coord::new(4, 2),
			Coord::new(3, 4),
			Coord::new(5, 4),
		];
		let expected = solver(start, splitters);

		assert_eq!(result.start, start, "Start point");
		assert_eq!(result, expected, "Complete solver");
	}

	fn solver(start: Coord, splitters: Vec<Coord>) -> Solver {
		Solver { start, splitters }
	}
}
