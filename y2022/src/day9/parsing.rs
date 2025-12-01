use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

use super::moves::{Direction, Move};

pub fn parse(input: &str) -> Vec<Move> {
	let moves: Result<_, _> = input.lines().map(|l| l.parse()).collect();
	moves.expect("Invalid moves")
}

impl FromStr for Move {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split_whitespace();
		let dir = parts.next().ok_or(ParseError::InvalidInput(s.to_string()))?;
		let steps = parts.next().ok_or(ParseError::InvalidInput(s.to_string()))?;

		let direction = match dir {
			"U" => Direction::Up,
			"D" => Direction::Down,
			"L" => Direction::Left,
			"R" => Direction::Right,
			x => Err(ParseError::DirectionError(x.to_string()))?,
		};

		let steps = steps.parse().map_err(|e| ParseError::IntError(e))?;
		let m = Move::new(direction, steps);
		Ok(m)
	}
}

#[derive(Debug)]
pub enum ParseError {
	DirectionError(String),
	IntError(ParseIntError),
	InvalidInput(String),
}

impl Error for ParseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ParseError::IntError(parse_int_error) => Some(parse_int_error),
			_ => None,
		}
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::DirectionError(d) => {
				writeln!(f, "Direction Error: '{}'", d)
			}
			ParseError::IntError(e) => {
				writeln!(f, "Int parsing error:\n{}", e)
			}
			ParseError::InvalidInput(i) => {
				writeln!(f, "Invalid input: '{}'", i)
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::moves::*;
	use indoc::indoc;

	#[test]
	fn move_parse() {
		let input = indoc! {"
			R 4
			U 4
			L 42
			D 1"};

		let result = parse(input);

		let expected = vec![
			Move(Direction::Right, 4),
			Move(Direction::Up, 4),
			Move(Direction::Left, 42),
			Move(Direction::Down, 1),
		];
		assert_eq!(result, expected);
	}

	#[allow(nonstandard_style)]
	fn Move(direction: Direction, steps: usize) -> Move {
		Move::new(direction, steps)
	}
}
