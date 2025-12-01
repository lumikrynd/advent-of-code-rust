use aoc_helpers::PuzzleSolver;

use rotations::Rotations;

use dial::Dial;

pub struct Solver {
	rotations: Rotations,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let rotations = rotations::parse(input);
		Box::new(Solver { rotations })
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let dial = dial::new(50);
		let result = super_secret_password(dial, &self.rotations);
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn super_secret_password(start_position: Dial, rotations: &Rotations) -> usize {
	let positions =
		rotations.iter().fold(vec![start_position], |mut acc, r| {
			let t = acc.last().unwrap();
			let next = t.rotate(r);
			acc.push(next);
			acc
		});

	positions.iter().filter(|p| p.position() == 0).count()
}

mod dial {
	use super::rotations::Direction::*;
	use super::rotations::Rotation;

	#[derive(Clone, Copy)]
	pub struct Dial {
		position: i16,
	}

	pub fn new(start: i16) -> Dial {
		let 0..100 = start else {
			panic!("Invalid input: {}", start)
		};

		Dial { position: start }
	}

	impl Dial {
		pub fn position(&self) -> i16 {
			self.position
		}

		pub fn rotate(&self, rotation: &Rotation) -> Self {
			let clicks = rotation.clicks();
			let temp = match rotation.direction() {
				Left => self.position - clicks,
				Right => self.position + clicks,
			};

			new(temp.rem_euclid(100))
		}
	}
}

mod rotations {
	pub fn parse(input: &str) -> Rotations {
		input.lines().map(parse_single).collect()
	}

	fn parse_single(input: &str) -> Rotation {
		let (dir, clicks) = input.split_at(1);
		let direction = match dir {
			"L" => Direction::Left,
			"R" => Direction::Right,
			_ => panic!("invalid direction"),
		};

		let clicks = clicks.parse().unwrap();

		new(direction, clicks)
	}

	pub type Rotations = Vec<Rotation>;

	#[derive(Debug, PartialEq)]
	pub struct Rotation {
		direction: Direction,
		clicks: i16,
	}

	impl Rotation {
		pub fn direction(&self) -> Direction {
			self.direction
		}

		pub fn clicks(&self) -> i16 {
			self.clicks
		}
	}

	pub fn new(direction: Direction, clicks: i16) -> Rotation {
		Rotation { direction, clicks }
	}

	#[derive(Clone, Copy, Debug, PartialEq)]
	pub enum Direction {
		Left,
		Right,
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	use super::rotations::Direction::*;
	use super::rotations::new as rotation;

	#[test]
	fn example_case() {
		let input = indoc! {"
			L68
			L30
			R48
			L5
			R60
			L55
			L1
			L99
			R14
			L82"};

		let solver = Solver::new(input);

		let result = solver.solve_part_1().unwrap();
		assert_eq!(result, "3");
	}

	#[test]
	fn parse() {
		let input = indoc! {"
			L68
			R42
			L30"};

		let result = rotations::parse(input);

		let expected = vec![
			rotation(Left, 68),
			rotation(Right, 42),
			rotation(Left, 30),
		];

		assert_eq!(result, expected);
	}

	#[test]
	fn rotate() {
		let d = dial::new(42);
		assert_eq!(2, d.rotate(&rotation(Left, 40)).position(), "left");
		assert_eq!(82, d.rotate(&rotation(Right, 40)).position(), "right");

		let msg = "left overflow";
		assert_eq!(62, d.rotate(&rotation(Left, 80)).position(), "{msg}");
		let msg = "right overflow";
		assert_eq!(22, d.rotate(&rotation(Right, 80)).position(), "{msg}");

		let msg = "Round trips";
		assert_eq!(40, d.rotate(&rotation(Left, 802)).position(), "{msg}");
	}
}
