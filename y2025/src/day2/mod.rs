use std::collections::HashSet;

use aoc_helpers::PuzzleSolver;

type Int = u64; //faster to change everywhere
type Range = std::ops::Range<Int>;

pub struct Solver {
	ranges: Vec<Range>,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let ranges = parse(input);
		Box::new(Solver { ranges })
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let result = sum_invalid_ids(&self.ranges, 2);
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let result = sum_invalid_ids_v2(&self.ranges);
		Some(result.to_string())
	}
}

fn sum_invalid_ids_v2(ranges: &[Range]) -> Int {
	ranges.iter().flat_map(get_all_invalid_ids).sum()
}

fn sum_invalid_ids(ranges: &[Range], parts: u32) -> Int {
	ranges.iter().flat_map(|v| get_invalid_ids(v, parts)).sum()
}

fn get_all_invalid_ids(range: &Range) -> HashSet<Int> {
	let max_parts = count_digits(range.end);
	(2..=max_parts)
		.flat_map(|p| get_invalid_ids(range, p))
		.collect()
}

fn get_invalid_ids(range: &Range, parts: u32) -> Vec<Int> {
	assert!(parts > 1);

	let first_digit = get_first_digit(range.start, parts);

	let mut result = vec![];
	for i in first_digit.. {
		let v = pattern_number(i, parts);
		if v > range.end {
			break;
		}

		if v >= range.start {
			result.push(v);
		}
	}

	result
}

fn get_first_digit(n: u64, parts: u32) -> u64 {
	let digits = count_digits(n);
	let size = digits / parts;

	if digits.is_multiple_of(parts) {
		n / pow10(size).pow(parts - 1)
	} else {
		pow10(size)
	}
}

fn pattern_number(n: u64, repeat: u32) -> u64 {
	let digits = count_digits(n);
	let mut res = n;
	for i in 1..repeat {
		res += n * pow10(digits * i);
	}
	res
}

fn pow10(n: u32) -> u64 {
	(10 as Int).pow(n)
}

fn count_digits(n: u64) -> u32 {
	n.ilog10() + 1
}

fn parse(input: &str) -> Vec<Range> {
	input.trim().split(',').map(parse_single).collect()
}

fn parse_single(input: &str) -> Range {
	let (start, end) = input.split_once('-').unwrap();
	let start = start.parse().unwrap_or_else(|_| panic!("'{}'", start));
	let end = end.parse().unwrap_or_else(|_| panic!("'{}'", end));
	Range { start, end }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn name() {
		let input = "11-22,95-115,998-1012,1188511880-1188511890,\
			222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
			565653-565659,824824821-824824827,2121212118-2121212124";

		let solver = Solver::new(input);

		let result = solver.solve_part_1();
		assert_eq!("1227775554", result.unwrap());

		let result = solver.solve_part_2();
		assert_eq!("4174379265", result.unwrap());
	}

	#[test]
	fn get_first_digit_test() {
		assert_eq!(2, get_first_digit(20, 2));
		assert_eq!(5, get_first_digit(59, 2));
		assert_eq!(1, get_first_digit(9, 2));
		assert_eq!(10, get_first_digit(911, 2));
		assert_eq!(2, get_first_digit(202, 3));
		assert_eq!(20, get_first_digit(200200, 3));
		assert_eq!(1, get_first_digit(100100, 6));
		assert_eq!(10, get_first_digit(10010, 3));
		assert_eq!(20, get_first_digit(20010000, 4));
	}

	#[test]
	fn pattern_number_test() {
		assert_eq!(22, pattern_number(2, 2));
		assert_eq!(1515, pattern_number(15, 2));
		assert_eq!(15151515, pattern_number(15, 4));
	}

	#[test]
	fn get_invalid_ids_test() {
		let value = range(10, 23);
		let result = get_invalid_ids(&value, 2);
		assert_eq!(result, [11, 22]);

		let value = range(100, 232);
		let result = get_invalid_ids(&value, 3);
		assert_eq!(result, [111, 222]);
	}

	#[test]
	fn get_all_invalid_ids_test() {
		let value = range(11, 22);
		let result = get_all_invalid_ids(&value);
		assert_eq!(result, HashSet::from([11, 22]));

		let value = range(95, 115);
		let result = get_all_invalid_ids(&value);
		assert_eq!(result, HashSet::from([99, 111]));

		let value = range(998, 1012);
		let result = get_all_invalid_ids(&value);
		assert_eq!(result, HashSet::from([999, 1010]));

		let value = range(222220, 222224);
		let result = get_all_invalid_ids(&value);
		assert_eq!(result, HashSet::from([222222]));
	}

	#[test]
	fn parse_test() {
		let input = "11-22,1188511880-1188511890";

		let result = parse(input);

		let expected = [
			range(11, 22),
			range(1188511880, 1188511890),
		];

		assert_eq!(result, expected);
	}

	fn range(start: Int, end: Int) -> Range {
		Range { start, end }
	}
}
