use aoc_helpers::{PuzzleSolver};

mod parsing;

pub struct Solver {
	moves: Vec<Move>,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		None
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let moves = parsing::parse(input);
		let s = Solver { moves };
		Box::new(s)
	}
}

#[derive(Debug, PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq)]
struct Move {
	direction: Direction,
	steps: usize,
}

#[cfg(test)]
mod test {
}
