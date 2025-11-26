use aoc_helpers::{PuzzleSolver, errors::AocError, wrapper};

mod parsing;

pub struct Solver {
	stacks: Vec<Stack>,
	moves: Vec<Move>,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		Some(self.solve_part_1_res().unwrap())
	}

	fn solve_part_2(&self) -> Option<String> {
		Some(self.solve_part_2_res().unwrap())
	}
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let solver = parsing::parse(input);
		Box::new(solver.expect("Well, if it isn't working"))
	}

	fn solve_part_1_res(&self) -> Result<String, AocError> {
		let mut state = State::new(&self.stacks);
		for m in &self.moves {
			for _ in 0..m.count {
				state.move_crates(m.from, m.to, 1)?
			}
		}

		state.get_final_result()
	}

	fn solve_part_2_res(&self) -> Result<String, AocError> {
		let mut state = State::new(&self.stacks);
		for m in &self.moves {
			state.move_crates(m.from, m.to, m.count)?
		}

		state.get_final_result()
	}
}

pub struct State {
	stacks: Vec<Stack>,
}

impl State {
	fn new(start: &Vec<Stack>) -> Self {
		let stacks = (*start).clone();
		State { stacks }
	}

	fn move_crates(&mut self, from: u8, to: u8, count: u8) -> Result<(), AocError> {
		let from = (from - 1) as usize;
		let to = (to - 1) as usize;
		let count = count as usize;

		let from_stack = &mut self.stacks[from];
		let length = from_stack.len();
		let mut crates_: Vec<_> = from_stack.drain(length - count..).collect();

		self.stacks[to].append(&mut crates_);
		Ok(())
	}

	fn get_final_result(&self) -> Result<String, AocError> {
		let result: String = self
			.stacks
			.iter()
			.map(|f| f.last())
			.map(|f| f.unwrap())
			.collect();

		Ok(result)
	}
}

wrapper!(Stack, Vec<char>, Debug);

#[derive(Debug, PartialEq)]
pub struct Move {
	from: u8,
	to: u8,
	count: u8,
}

impl Move {
	fn new(from: u8, to: u8, count: u8) -> Self {
		Move { from, to, count }
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::error::Error;

	#[test]
	fn move_crates_test() -> Result<(), Box<dyn Error>> {
		let input = vec![
			Stack(vec!['A', 'B', 'C', 'D', 'E']),
			Stack(vec!['F', 'G', 'H', 'I', 'J']),
		];

		let mut state = State::new(&input);
		state.move_crates(1, 2, 1)?;
		assert_eq!(state.get_final_result()?, "DE");

		let mut state = State::new(&input);
		state.move_crates(2, 1, 1)?;
		assert_eq!(state.get_final_result()?, "JI");

		let mut state = State::new(&input);
		state.move_crates(2, 1, 2)?;
		assert_eq!(state.get_final_result()?, "JH");

		Ok(())
	}
}
