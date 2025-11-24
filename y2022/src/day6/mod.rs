use std::collections::HashSet;

use aoc_helpers::PuzzleSolver;

pub struct Solver(String);

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let end_index = find_3_diff_point(&self.0).unwrap();
		Some(end_index.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let end_index = find_14_diff_point(&self.0).unwrap();
		Some(end_index.to_string())
	}
}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		Box::new(Solver(input.to_string()))
	}
}

fn find_3_diff_point(s: &str) -> Option<usize> {
	find_x_diff_point(s, 4)
}

fn find_14_diff_point(s: &str) -> Option<usize> {
	find_x_diff_point(s, 14)
}

fn find_x_diff_point(s: &str, area_size: usize) -> Option<usize> {
	for i in 0..=(s.len() - area_size) {
		let sub_area = &s[i..i + area_size];
		let characters: HashSet<_> = sub_area.chars().collect();
		if characters.len() == area_size {
			return Some(i + area_size);
		}
	}

	None
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn things() {
		let input = "abcd";
		assert_eq!(find_3_diff_point(input), Some(4));

		let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
		assert_eq!(find_3_diff_point(input), Some(5));

		let input = "nppdvjthqldpwncqszvftbrmjlhg";
		assert_eq!(find_3_diff_point(input), Some(6));

		let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
		assert_eq!(find_3_diff_point(input), Some(10));

		let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
		assert_eq!(find_3_diff_point(input), Some(11));
	}
}
