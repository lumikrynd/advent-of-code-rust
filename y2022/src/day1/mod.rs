use std::collections::BinaryHeap;

use aoc_helpers::PuzzleSolver;

mod parsing;

fn solve_part_1(elfs: &[Elf]) -> String {
	let result = elfs.iter().map(|e| e.sum()).max().unwrap();

	result.to_string()
}

fn solve_part_2(elfs: &[Elf]) -> String {
	let values = elfs.iter().map(|e| e.sum());
	let sorted = BinaryHeap::from_iter(values);

	let result: u32 = sorted.iter().take(3).sum();
	result.to_string()
}

#[derive(Debug)]
struct Elf {
	food: Vec<u32>,
}

impl Elf {
	fn sum(&self) -> u32 {
		self.food.iter().sum()
	}
}

pub struct Solver {
	elfs: Vec<Elf>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let elfs = parsing::parse(input);
		Box::new(Solver { elfs })
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let s = solve_part_1(&self.elfs);
		Some(s)
	}

	fn solve_part_2(&self) -> Option<String> {
		let s = solve_part_2(&self.elfs);
		Some(s)
	}
}
