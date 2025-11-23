use aoc_helpers::PuzzleSolver;
use parsing::parse;

mod parsing;

type ElfPair = (Range, Range);

pub struct Solver {
	elf_pairs: Vec<ElfPair>,
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let sum = self
			.elf_pairs
			.iter()
			.filter(|x| one_contains_other(x))
			.count();
		Some(sum.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let sum = self.elf_pairs.iter().filter(|x| x.0.overlaps(&x.1)).count();
		Some(sum.to_string())
	}
}

fn one_contains_other(pair: &ElfPair) -> bool {
	pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0)
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let elf_pairs = parse(input);
		Box::new(Solver { elf_pairs })
	}
}

#[derive(Debug, PartialEq)]
struct Range {
	start: u32,
	end: u32,
}

impl Range {
	fn new(start: u32, end: u32) -> Range {
		assert!(start <= end);
		Range { start, end }
	}

	fn fully_contains(&self, other: &Self) -> bool {
		self.contains(&other.start) && self.contains(&other.end)
	}

	fn overlaps(&self, other: &Self) -> bool {
		self.contains(&other.start)
			|| self.contains(&other.end)
			|| other.contains(&self.start)
			|| other.contains(&self.end)
	}

	fn contains(&self, point: &u32) -> bool {
		&self.start <= point && &self.end >= point
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_fully_contains() {
		assert!(!Range::new(1, 2).fully_contains(&Range::new(3, 4)));
		assert!(Range::new(1, 2).fully_contains(&Range::new(2, 2)));
	}

	#[test]
	fn test_overlaps() {
		assert!(!Range::new(1, 2).overlaps(&Range::new(3, 4)));

		let a = Range::new(1, 2);
		let b = Range::new(2, 3);
		assert!(a.overlaps(&b));
		assert!(b.overlaps(&a));

		let a = Range::new(1, 4);
		let b = Range::new(2, 3);
		assert!(a.overlaps(&b));
		assert!(b.overlaps(&a));
	}
}
