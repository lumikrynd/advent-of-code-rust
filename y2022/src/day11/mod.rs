use std::collections::HashMap;

use aoc_helpers::PuzzleSolver;
use monkeys::*;

mod monkeys;

pub struct Solver {
	monkeys: Vec<Monkey>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let solver = Solver {
			monkeys: parse(input),
		};
		Box::new(solver)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let count = part_1(&self.monkeys);
		Some(count.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn part_1(monkeys: &Vec<Monkey>) -> usize {
	let mut items: HashMap<_, _> = monkeys
		.iter()
		.map(|m| (m.index, m.starting_items.clone()))
		.collect();

	let iter = (0..20).flat_map(|_| monkeys.iter());

	let mut counts = vec![0; monkeys.len()];

	for monkey in iter {
		while let Some(item) = items.get_mut(&monkey.index).unwrap().pop() {
			let item = monkey.operation.execute(item);
			let item = item / 3;
			let next = monkey.test.next_monkey(item);
			items.get_mut(&next).unwrap().push(item);

			counts[monkey.index as usize] += 1;
		}
	}

	counts.sort_unstable();
	counts.pop().unwrap() * counts.pop().unwrap()
}

impl Operation {
	fn execute(&self, i: usize) -> usize {
		match self {
			Operation::Multiply(v) => i * v,
			Operation::Add(v) => i + v,
			Operation::Power => i * i,
		}
	}
}

impl Test {
	fn next_monkey(&self, item: usize) -> u8 {
		if item.is_multiple_of(self.div) {
			self.on_true
		} else {
			self.on_false
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	use aoc_helpers::PuzzleSolver;
	use indoc::indoc;

	#[test]
	fn part_1_test() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "10605");
	}

	const EXAMPLE: &str = indoc! {"
		Monkey 0:
		  Starting items: 79, 98
		  Operation: new = old * 19
		  Test: divisible by 23
		    If true: throw to monkey 2
		    If false: throw to monkey 3

		Monkey 1:
		  Starting items: 54, 65, 75, 74
		  Operation: new = old + 6
		  Test: divisible by 19
		    If true: throw to monkey 2
		    If false: throw to monkey 0

		Monkey 2:
		  Starting items: 79, 60, 97
		  Operation: new = old * old
		  Test: divisible by 13
		    If true: throw to monkey 1
		    If false: throw to monkey 3

		Monkey 3:
		  Starting items: 74
		  Operation: new = old + 3
		  Test: divisible by 17
		    If true: throw to monkey 0
		    If false: throw to monkey 1"};
}
