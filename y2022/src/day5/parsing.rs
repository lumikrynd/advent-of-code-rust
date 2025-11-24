use aoc_helpers::errors::AocError;
use std::error::Error;

use super::*;

mod input_split;

pub fn parse(input: &str) -> Result<Solver, Box<dyn Error>> {
	let mut sections = input_split::split_into_sections(input).into_iter();
	let stacks = sections.next().unwrap();
	let moves = sections.next().unwrap();

	let stacks = parse_stacks(&stacks)?;
	let moves = parse_moves(&moves)?;

	Ok(Solver { stacks, moves })
}

fn parse_stacks(stacks: &Vec<&str>) -> Result<Vec<Stack>, Box<dyn Error>> {
	let mut stacks = stacks.iter().rev();
	let def = stacks.next().ok_or(AocError::new("no definition line"))?;

	let vectors = stacks.fold(initialize_stacks(def), |acc, val| insert_boxes(acc, val));

	Ok(vectors)
}

fn insert_boxes(mut stacks: Vec<Stack>, row: &str) -> Vec<Stack> {
	let chars: Vec<char> = row.chars().collect();
	for (i, stack) in stacks.iter_mut().enumerate() {
		let j = 1 + 4 * i;
		if chars[j] != ' ' {
			stack.0.push(chars[j]);
		}
	}
	stacks
}

fn initialize_stacks(def: &str) -> Vec<Stack> {
	let length = (def.len() + 1) / 4;
	let mut stacks = Vec::with_capacity(length);
	for _ in 0..length {
		stacks.push(Stack(Vec::new()));
	}
	stacks
}

fn parse_moves(moves: &Vec<&str>) -> Result<Vec<Move>, Box<dyn Error>> {
	moves.iter().map(|k| parse_move(k)).collect()
}

fn parse_move(m: &str) -> Result<Move, Box<dyn Error>> {
	let parts: Vec<_> = m.split(' ').collect();
	let ["move", count, "from", from, "to", to] = parts[..] else {
		return Err(AocError::boxed(&format!("Didn't match move pattern:\n{m}")));
	};

	let res = Move::new(from.parse()?, to.parse()?, count.parse()?);
	Ok(res)
}

#[cfg(test)]
mod test {
	use std::result;

use super::*;

	#[test]
	fn parse_move_test() {
		let moves = vec![
			"move 1 from 2 to 3",
			"move 5 from 3 to 1",
		];
		let moves = parse_moves(&moves).unwrap();

		let expected = vec![
			Move::new(2, 3, 1),
			Move::new(3, 1, 5),
		];

		assert_eq!(moves, expected);
	}

	#[test]
	fn parse_stacks_test() -> Result<(), Box<dyn Error>> {
		let input = vec![
			"    [D]    ",
			"[N] [C]    ",
			"[Z] [M] [P]",
			" 1   2   3 ",
		];

		let result = parse_stacks(&input)?;

		let expected = vec![
			Stack(vec!['Z', 'N']),
			Stack(vec!['M', 'C', 'D']),
			Stack(vec!['P']),
		];

		assert_eq!(result.len(), 3);
		assert_eq!(result, expected);

		Ok(())
	}

	#[test]
	fn full_parse() {
		let input = String::new() +
			"    [D]    \n" +
			"[N] [C]    \n" +
			"[Z] [M] [P]\n" +
			" 1   2   3 \n" +
			"\n" +
			"move 1 from 2 to 3\n" +
			"move 5 from 3 to 1" ;

		let Solver{stacks, moves} = parse(&input).unwrap();

		let expected = vec![
			Stack(vec!['Z', 'N']),
			Stack(vec!['M', 'C', 'D']),
			Stack(vec!['P']),
		];

		assert_eq!(stacks, expected);

		let expected = vec![
			Move::new(2, 3, 1),
			Move::new(3, 1, 5),
		];

		assert_eq!(moves, expected);
	}
}
