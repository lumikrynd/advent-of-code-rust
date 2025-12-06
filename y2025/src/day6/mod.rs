use aoc_helpers::PuzzleSolver;

type Number = usize;

#[derive(Debug, PartialEq)]
pub struct Solver {
	numbers: Vec<Vec<Number>>,
	operators: Vec<Operator>,
}

#[derive(Debug, PartialEq)]
enum Operator {
	Add,
	Multiply,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = parse(input);
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let calculatins = self.operators.len();

		let mut res = 0;
		for index in 0..calculatins {
			let opp = get_opp(&self.operators[index]);

			res += self
				.numbers
				.iter()
				.map(|l| l[index])
				.reduce(|acc, new| opp(acc, new))
				.expect("List shouldn't be empty");
		}

		return Some(res.to_string());
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn get_opp(operator: &Operator) -> impl Fn(Number, Number) -> Number {
	match operator {
		Operator::Add => |a, b| a + b,
		Operator::Multiply => |a, b| a * b,
	}
}

fn parse(input: &str) -> Solver {
	let mut it = input.lines().rev();
	let operators = it.next().expect("Missing operator line");
	let numbers = it.rev();

	let operators = operators.split_whitespace().map(parse_operator).collect();
	let numbers = numbers.map(parse_number_row).collect();

	return Solver { numbers, operators };

	fn parse_number_row(row: &str) -> Vec<Number> {
		row.split_whitespace()
			.map(|n| n.parse().expect("invalid number"))
			.collect()
	}

	fn parse_operator(i: &str) -> Operator {
		match i {
			"+" => Operator::Add,
			"*" => Operator::Multiply,
			_ => panic!(""),
		}
	}
}

#[cfg(test)]
mod test {
	use super::Operator::*;
	use super::*;
	use indoc::indoc;

	#[test]
	fn part_1() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap(), "4277556")
	}

	#[test]
	fn parse_empty() {
		let result = parse("\n");
		assert_eq!(result, solver(vec![], vec![]))
	}

	#[test]
	fn parse_example() {
		let result = parse(EXAMPLE);

		let expected = solver(
			vec![
				vec![123, 328, 51, 64],
				vec![45, 64, 387, 23],
				vec![6, 98, 215, 314],
			],
			vec![Multiply, Add, Multiply, Add],
		);

		assert_eq!(result, expected)
	}

	fn solver(numbers: Vec<Vec<Number>>, operators: Vec<Operator>) -> Solver {
		Solver { numbers, operators }
	}

	const EXAMPLE: &str = indoc! {"
		123 328  51 64 
		 45 64  387 23 
		  6 98  215 314
		*   +   *   +  "};
}
