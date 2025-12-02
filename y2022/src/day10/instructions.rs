use super::Int;

#[derive(Debug, PartialEq)]
pub enum Instruction {
	Noop,
	Addx(Int),
}

pub type Program = Vec<Instruction>;

use Instruction::*;

pub fn parse(input: &str) -> Program {
	input.lines().map(parse_instruction).collect()
}

fn parse_instruction(input: &str) -> Instruction {
	if let Some(a) = input.strip_prefix("addx ") {
		let a: Int = a.parse().unwrap();
		Addx(a)
	} else if input == "noop" {
		Noop
	} else {
		panic!("Invalid input '{}'", input)
	}
}


#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn parse_test() {
		let input = indoc! {"
		noop
		addx 3
		addx -5"};

		let result = parse(input);

		assert_eq!(result, [Noop, Addx(3), Addx(-5)]);
	}
}
