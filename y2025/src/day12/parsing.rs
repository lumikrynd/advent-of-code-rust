use super::*;

pub fn parse(input: &str) -> Solver {
	let (shapes, regions) = input.rsplit_once("\n\n").unwrap();

	Solver {
		shapes: parse_shapes(shapes),
		regions: parse_regions(regions),
	}
}

fn parse_shapes(input: &str) -> Vec<Shape> {
	input.split("\n\n").map(parse_shape).collect()
}

fn parse_shape(input: &str) -> Shape {
	return Shape {
		fields: input.lines().skip(1).map(map_line).collect(),
	};

	fn map_line(i: &str) -> Vec<bool> {
		i.chars().map(map_char).collect()
	}

	fn map_char(c: char) -> bool {
		match c {
			'#' => true,
			'.' => false,
			_ => panic!(""),
		}
	}
}

fn parse_regions(input: &str) -> Vec<RegionSpec> {
	input.lines().map(parse_region).collect()
}

fn parse_region(input: &str) -> RegionSpec {
	let (dimensions, counts) = input.split_once(": ").unwrap();

	return RegionSpec {
		dimensions: parse_dim(dimensions),
		counts: parse_counts(counts),
	};

	fn parse_dim(input: &str) -> (u8, u8) {
		let (a, b) = input.split_once('x').unwrap();
		(a.parse().unwrap(), b.parse().unwrap())
	}

	fn parse_counts(counts: &str) -> Vec<usize> {
		counts.split(' ').map(|f| f.parse().unwrap()).collect()
	}
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn parsing() {
		let input = indoc! {"
			0:
			###
			##.
			##.

			0:
			#.#
			.#.
			##.

			4x5: 0 0 0 0 2 0
			1x2: 1 0 0 0 0 0"};

		let expected = Solver {
			shapes: vec![
				Shape {
					fields: vec![
						vec![true, true, true],
						vec![true, true, false],
						vec![true, true, false],
					],
				},
				Shape {
					fields: vec![
						vec![true, false, true],
						vec![false, true, false],
						vec![true, true, false],
					],
				},
			],
			regions: vec![
				RegionSpec {
					dimensions: (4, 5),
					counts: vec![0, 0, 0, 0, 2, 0],
				},
				RegionSpec {
					dimensions: (1, 2),
					counts: vec![1, 0, 0, 0, 0, 0],
				},
			],
		};

		let result = parse(input);

		assert_eq!(result.shapes, expected.shapes, "shapes error");
		assert_eq!(result.regions, expected.regions, "regions error");
		assert_eq!(result, expected, "full error");
	}
}
