use aoc_helpers::PuzzleSolver;

type Number = usize;

#[derive(Debug, PartialEq)]
pub struct Solver<'l> {
	numbers: Vec<Vec<&'l str>>,
	operators: Vec<Operator>,
}

#[derive(Debug, PartialEq)]
enum Operator {
	Add,
	Multiply,
}

impl<'l> Solver<'l> {
	pub fn new(input: &'l str) -> Box<Self> {
		let s = parse(input);
		Box::new(s)
	}
}

impl<'l> PuzzleSolver for Solver<'l> {
	fn solve_part_1(&self) -> Option<String> {
		let numbers = part_1_parse(&self.numbers);
		let res = calculate(numbers, &self.operators);
		Some(res.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let numbers = part_2_parse(&self.numbers);
		let res = calculate(numbers, &self.operators);
		Some(res.to_string())
	}
}

fn part_1_parse(raw: &[Vec<&str>]) -> Vec<Vec<Number>> {
	let transposed = transpose(raw);
	return transposed.iter().map(parse_row).collect();

	fn parse_row(row: &Vec<&str>) -> Vec<Number> {
		row.iter().map(parse_num).collect()
	}

	fn parse_num(num: &&str) -> Number {
		num.trim().parse().expect("invalid number")
	}
}

fn part_2_parse(raw: &[Vec<&str>]) -> Vec<Vec<usize>> {
	let transposed = transpose(raw);
	return transposed.iter().map(parse_row).collect();

	fn parse_row(row: &Vec<&str>) -> Vec<Number> {
		let width = row[0].chars().count();

		(0..width)
			.rev()
			.map(|i| recreate_number(row, i))
			.map(|s| parse_num(&s))
			.collect()
	}

	fn recreate_number(row: &[&str], column: usize) -> String {
		row.iter()
			.map(|s| s.chars().nth(column).unwrap())
			.collect()
	}

	fn parse_num(num: &str) -> Number {
		num.trim().parse().expect("invalid number")
	}
}

fn transpose<'l>(raw: &[Vec<&'l str>]) -> Vec<Vec<&'l str>> {
	let problem_length = raw.len();
	let problems = raw.first().map(|x| x.len()).unwrap_or(0);

	(0..problems)
		.map(|p| (0..problem_length).map(|l| raw[l][p]).collect())
		.collect()
}

fn calculate(numbers: Vec<Vec<usize>>, operators: &[Operator]) -> usize {
	return numbers.iter().zip(operators).map(single_calculation).sum();

	fn single_calculation((n, o): (&Vec<usize>, &Operator)) -> usize {
		let op = get_operation(o);
		n.iter().cloned().reduce(op).unwrap_or(0)
	}
}

fn get_operation(operator: &Operator) -> impl Fn(Number, Number) -> Number {
	match operator {
		Operator::Add => |a, b| a + b,
		Operator::Multiply => |a, b| a * b,
	}
}

fn parse<'l>(input: &'l str) -> Solver<'l> {
	let mut it = input.lines().rev();
	let op_row = it.next().expect("Missing operator line");
	let numbers = it.rev();

	let operators = op_row.split_whitespace().map(parse_operator).collect();

	let mut column_widths: Vec<_> = op_row
		.split(['*', '+'])
		.skip(1)
		.map(|x| x.chars().count())
		.collect();

	if let Some(k) = column_widths.last_mut() {
		*k += 1;
	}

	let numbers = numbers
		.map(|s| parse_number_row(s, &column_widths))
		.collect();

	return Solver { numbers, operators };

	fn parse_number_row<'l>(
		mut row: &'l str,
		widths: &[usize],
	) -> Vec<&'l str> {
		widths.iter().fold(vec![], move |mut acc, w| {
			acc.push(&row[0..*w]);
			row = match row.split_at_checked(*w + 1) {
				Some((_, r)) => r,
				None => "",
			};
			acc
		})
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
	fn part_2() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_2().unwrap(), "3263827")
	}

	#[test]
	fn parse_empty() {
		let result = parse("\n");
		assert_eq!(result, solver(vec![], vec![]))
	}

	#[test]
	fn part_1_parse_test() {
		let expected = vec![
			vec![123, 45, 6],
			vec![328, 64, 98],
			vec![51, 387, 215],
			vec![64, 23, 314],
		];
		let raw = vec![
			vec!["123", "328", " 51", "64 "],
			vec![" 45", "64 ", "387", "23 "],
			vec!["  6", "98 ", "215", "314"],
		];

		assert_eq!(part_1_parse(&raw), expected);
	}

	#[test]
	fn parse_example() {
		let result = parse(EXAMPLE);

		let expected = solver(
			vec![
				vec!["123", "328", " 51", "64 "],
				vec![" 45", "64 ", "387", "23 "],
				vec!["  6", "98 ", "215", "314"],
			],
			vec![Multiply, Add, Multiply, Add],
		);

		assert_eq!(result, expected)
	}

	fn solver<'l>(
		numbers: Vec<Vec<&'l str>>,
		operators: Vec<Operator>,
	) -> Solver<'l> {
		Solver { numbers, operators }
	}

	const EXAMPLE: &str = indoc! {"
		123 328  51 64 
		 45 64  387 23 
		  6 98  215 314
		*   +   *   +  "};
}
