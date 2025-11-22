use crate::day4::parsing::parse;
use aoc_helpers::PuzzleSolver;

mod parsing;

type ElfPair = (Range, Range);

pub struct Solver {
    elf_pairs: Vec<ElfPair>,
}

impl PuzzleSolver for Solver {
    fn solve_part_1(&self) -> Option<String> {
        let sum = self.elf_pairs.iter().filter(|x| one_contains_other(x)).count();
        Some(sum.to_string())
    }

    fn solve_part_2(&self) -> Option<String> {
        None
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
        self.start <= other.start && self.end >= other.end
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
}
