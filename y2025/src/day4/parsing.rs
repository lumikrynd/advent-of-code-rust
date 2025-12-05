pub fn parse(input: &str) -> Vec<Vec<bool>> {
	input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<bool> {
	input.chars().map(|c| match c {
		|'.' => false,
		|'@' => true,
		| _ => panic!("invalid"),
	}).collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_empty() {
		let input = "";
		let expected: Vec<Vec<bool>> = vec![];
		assert_eq!(parse(input), expected);
	}

	#[test]
	fn parse_all_empty() {
		let input = "...";
		let expected = vec![vec![false, false, false]];
		assert_eq!(parse(input), expected);
	}

	#[test]
	fn parse_full_example() {
		let input = "..@\n@..\n@@.";
		let expected = vec![
			vec![false, false, true],
			vec![true, false, false],
			vec![true, true, false],
		];
		assert_eq!(parse(input), expected);
	}
}
