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
		let dial = dial::new(50);
		let result = super_secret_password_2(dial, &self.rotations);
		Some(result.to_string())
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

fn super_secret_password_2(
	start_position: Dial,
	rotations: &Rotations,
) -> usize {
	let mut position = start_position;
	rotations
		.iter()
		.flat_map(|r| r.to_single_step())
		.fold(0, |acc, r| {
			position = position.rotate(&r);
			if position.position() == 0 {
				acc + 1
			} else {
				acc
			}
		})
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

	#[derive(Clone, Debug, PartialEq)]
	pub struct Rotation {
		direction: Direction,
		clicks: i16,
	}

	pub fn new(direction: Direction, clicks: i16) -> Rotation {
		Rotation { direction, clicks }
	}

	impl Rotation {
		pub fn direction(&self) -> Direction {
			self.direction
		}

		pub fn clicks(&self) -> i16 {
			self.clicks
		}

		pub fn to_single_step(&self) -> RotationsInSingleStep {
			RotationsInSingleStep {
				direction: self.direction,
				remaining: self.clicks,
			}
		}
	}

	pub struct RotationsInSingleStep {
		direction: Direction,
		remaining: i16,
	}

	impl Iterator for RotationsInSingleStep {
		type Item = Rotation;

		fn next(&mut self) -> Option<Self::Item> {
			if self.remaining <= 0 {
				return None;
			}

			self.remaining -= 1;
			Some(Self::Item {
				direction: self.direction,
				clicks: 1,
			})
		}
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

		let result = solver.solve_part_2().unwrap();
		assert_eq!(result, "6");
	}

	#[test]
	fn single_step_rotation() {
		let mut iter = rotation(Left, 3).to_single_step();

		assert_eq!(rotation(Left, 1), iter.next().unwrap());
		assert_eq!(rotation(Left, 1), iter.next().unwrap());
		assert_eq!(rotation(Left, 1), iter.next().unwrap());
		assert_eq!(None, iter.next());

		let mut iter = rotation(Right, 1).to_single_step();
		assert_eq!(rotation(Right, 1), iter.next().unwrap());
		assert_eq!(None, iter.next());
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
