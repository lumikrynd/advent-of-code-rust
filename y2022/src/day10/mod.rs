use aoc_helpers::PuzzleSolver;
use core::str;
use instructions::Instruction::*;
use instructions::*;

mod instructions;

type Int = isize;

pub struct Solver {
	program: Program,
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let solver = Solver {
			program: instructions::parse(input),
		};
		Box::new(solver)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let states = find_relevant_states(&self.program);
		let signal_strengths = states
			.iter()
			.enumerate()
			.map(|(i, s)| (i.cast_signed(), s))
			.inspect(print_states)
			.map(|(i, s)| part_1_formula(i, s));

		let sum: Int = signal_strengths.sum();
		Some(sum.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn print_states(&(i, s): &(Int, &State)) {
	println!(
		"{i}: At {} x_reg {}, would give {}",
		s.time,
		s.x_reg,
		part_1_formula(i, s)
	);
}

fn part_1_formula(i: Int, s: &State) -> Int {
	s.x_reg * (20 + 40 * i)
}

fn find_relevant_states(program: &[Instruction]) -> Vec<State> {
	let mut target = 20;
	let mut result = vec![];
	let mut previous = State::initial();

	for instruction in program {
		let current = previous.run(instruction);
		if current.time > target {
			result.push(previous);
			target += 40;
		}
		previous = current;
	}

	result
}

#[derive(Debug, PartialEq)]
struct State {
	time: usize,
	x_reg: Int,
}

impl State {
	fn initial() -> State {
		State { time: 1, x_reg: 1 }
	}

	fn run(&self, instruction: &Instruction) -> State {
		match instruction {
			Noop => State {
				time: self.time + 1,
				..*self
			},
			Addx(v) => State {
				time: self.time + 2,
				x_reg: self.x_reg + v,
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn state_next() {
		let mut s = State::initial();
		s = s.run(&Noop);
		assert_eq!(s, n_state(2, 1));
		s = s.run(&Addx(-3));
		assert_eq!(s, n_state(4, -2));
		s = s.run(&Addx(10));
		assert_eq!(s, n_state(6, 8));
	}

	#[test]
	fn find_every_20th_state_test() {
		let program = [
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
			Addx(2),
		];

		let states = find_relevant_states(&program);
		assert_eq!(states.len(), 1);
		assert_eq!(states[0].time, 19);
		assert_eq!(states[0].x_reg, 19);
	}

	fn n_state(time: usize, x_reg: Int) -> State {
		State { time, x_reg }
	}

	#[test]
	fn part_1_test() {
		let input = "\
			addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\n\
			addx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\n\
			addx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\n\
			addx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\n\
			addx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\n\
			noop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\n\
			addx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\n\
			addx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\n\
			noop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\n\
			noop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\n\
			noop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\n\
			addx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\n\
			addx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\n\
			noop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\n\
			addx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\n\
			noop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\n\
			noop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\n\
			noop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\n\
			addx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\n\
			addx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

		let solver = Solver {
			program: instructions::parse(input),
		};

		assert_eq!(solver.solve_part_1().unwrap(), "13140")
	}
}
