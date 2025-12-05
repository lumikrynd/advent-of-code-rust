use super::Range;
use super::Solver;

pub fn parse(input: &str) -> Solver {
	let mut lines = input.lines();
	let fresh = parse_fresh(&mut lines);
	let ingredients = parse_ingredients(&mut lines);

	Solver { fresh, ingredients }
}

fn parse_fresh<'l>(lines: &mut impl Iterator<Item = &'l str>) -> Vec<Range> {
	let mut result = Vec::new();

	for line in lines {
		if line.is_empty() {
			break;
		}

		let (from, to) = split_first(line);
		let (from, to) = (from.parse().expect(from), to.parse().expect(to));
		result.push(from..=to);
	}

	result
}

fn parse_ingredients<'l>(
	lines: &mut impl Iterator<Item = &'l str>,
) -> Vec<usize> {
	lines.map(|l| l.parse().unwrap()).collect()
}

fn split_first(line: &str) -> (&str, &str) {
	let (from, rest) = line.split_at(line.find('-').unwrap());
	let (_, to) = rest.split_at(1);
	(from, to)
}

#[cfg(test)]
mod test {
	use super::super::Id;
	use super::*;

	#[test]
	fn parse_empty_ish() {
		assert_eq!(parse("\n"), solver(vec![], vec![]));
	}

	#[test]
	fn parse_fresh() {
		let input = "3-5\n10-45\n\n";
		let result = parse(input);
		let expected = solver(vec![3..=5, 10..=45], vec![]);

		assert_eq!(result, expected);
	}

	#[test]
	fn parse_fruits() {
		let input = "\n3\n5\n10\n45";
		let result = parse(input);
		let expected = solver(vec![], vec![3, 5, 10, 45]);

		assert_eq!(result, expected);
	}

	fn solver(fresh: Vec<Range>, ingredients: Vec<Id>) -> Solver {
		Solver { fresh, ingredients }
	}
}
