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
		let result = sum_invalid_ids(&self.ranges);
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn sum_invalid_ids(ranges: &[Range]) -> Int {
	ranges.iter().flat_map(get_invalid_ids).sum()
}

fn get_invalid_ids(range: &Range) -> Vec<Int> {
	let first_digit = get_first_digit(range.start);

	let mut result = vec![];
	for i in first_digit.. {
		let v = pattern_number(i);
		if v > range.end {
			break;
		}

		if v >= range.start {
			result.push(v);
		}
	}

	result
}

fn get_first_digit(n: u64) -> u64 {
	let digits = count_digits(n);
	let half = digits / 2;

	if digits.is_multiple_of(2) {
		n / pow10(half)
	} else {
		pow10(half)
	}
}

fn pattern_number(n: u64) -> u64 {
	let digits = count_digits(n);
	n + n * pow10(digits)
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
	}

	#[test]
	fn get_first_digit_test() {
		assert_eq!(2, get_first_digit(20));
		assert_eq!(5, get_first_digit(59));
		assert_eq!(1, get_first_digit(9));
		assert_eq!(10, get_first_digit(911));
	}

	#[test]
	fn pattern_number_test() {
		assert_eq!(22, pattern_number(2));
		assert_eq!(1515, pattern_number(15));
	}

	#[test]
	fn get_invalid_ids_test() {
		let range = range(10, 23);
		let result = get_invalid_ids(&range);

		assert_eq!(vec![11, 22], result);
	}

	#[test]
	fn parse_test() {
		let input = "11-22,1188511880-1188511890";

		let result = parse(input);

		let expected = vec![
			range(11, 22),
			range(1188511880, 1188511890),
		];

		assert_eq!(expected, result);
	}

	fn range(start: Int, end: Int) -> Range {
		Range { start, end }
	}
}
