use std::{fmt::Debug, str::FromStr};

use Operation::*;

pub fn parse(input: &str) -> Vec<Monkey> {
	input.split("\n\n").map(parse_single).collect()
}

fn parse_single(input: &str) -> Monkey {
	let mut lines = input.trim().lines();

	let id = lines.next().unwrap();
	let id = strip_parse(id, "Monkey ", ":");

	let items = lines.next().unwrap().trim();
	let items = parse_items(items);

	let operation = lines.next().unwrap().trim();
	let operation = parse_operation(operation);

	let test_div = lines.next().unwrap().trim();
	let test_div = strip_parse(test_div, "Test: divisible by ", "");

	let on_true = lines.next().unwrap().trim();
	let on_true = strip_parse(on_true, "If true: throw to monkey ", "");

	let on_false = lines.next().unwrap().trim();
	let on_false = strip_parse(on_false, "If false: throw to monkey ", "");

	let test = Test::new(test_div, on_true, on_false);
	Monkey::new(id, items, operation, test)
}

fn parse_items(input: &str) -> Vec<usize> {
	let input = input.strip_prefix("Starting items: ").unwrap();
	input.split(", ").map(|s| s.parse().unwrap()).collect()
}

fn parse_operation(input: &str) -> Operation {
	let input = input.strip_prefix("Operation: new = old ").unwrap();
	if input == "* old" {
		Power
	} else if let Some(a) = input.strip_prefix("* ") {
		Multiply(a.parse().unwrap())
	} else if let Some(a) = input.strip_prefix("+ ") {
		Add(a.parse().unwrap())
	} else {
		panic!("Unsupported pattern: {}", input)
	}
}

#[derive(Debug, PartialEq)]
pub struct Monkey {
	pub index: u8,
	pub starting_items: Vec<usize>,
	pub operation: Operation,
	pub test: Test,
}

#[derive(Debug, PartialEq)]
pub struct Test {
	pub div: usize,
	pub on_true: u8,
	pub on_false: u8,
}

impl Test {
	fn new(div: usize, on_true: u8, on_false: u8) -> Self {
		Self {
			div,
			on_true,
			on_false,
		}
	}
}

impl Monkey {
	fn new(
		index: u8,
		starting_items: Vec<usize>,
		operation: Operation,
		test: Test,
	) -> Self {
		Self {
			index,
			starting_items,
			operation,
			test,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Operation {
	Multiply(usize),
	Add(usize),
	Power,
}

fn strip_parse<T>(text: &str, prefix: &str, suffix: &str) -> T
where
	T: FromStr,
	T::Err: Debug,
{
	strip(text, prefix, suffix).unwrap().parse::<T>().unwrap()
}

fn strip<'a>(text: &'a str, prefix: &str, suffix: &str) -> Option<&'a str> {
	let a = text.strip_prefix(prefix)?;
	let b = a.strip_suffix(suffix)?;
	Some(b)
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn parse_test() {
		let input = indoc! {"
			Monkey 0:
			  Starting items: 79, 98
			  Operation: new = old * 19
			  Test: divisible by 23
			    If true: throw to monkey 2
			    If false: throw to monkey 3

			Monkey 1:
			  Starting items: 54, 65, 75, 74
			  Operation: new = old + 6
			  Test: divisible by 19
			    If true: throw to monkey 2
			    If false: throw to monkey 0

			Monkey 2:
			  Starting items: 79, 60, 97
			  Operation: new = old * old
			  Test: divisible by 13
			    If true: throw to monkey 1
			    If false: throw to monkey 3"};

		let result = parse(input);

		let expected = vec![
			Monkey::new(0, vec![79, 98], Multiply(19), Test::new(23, 2, 3)),
			Monkey::new(1, vec![54, 65, 75, 74], Add(6), Test::new(19, 2, 0)),
			Monkey::new(2, vec![79, 60, 97], Power, Test::new(13, 1, 3)),
		];

		assert_eq!(result, expected);
	}
}
