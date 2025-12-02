use super::Elf;

enum Token {
	Empty,
	Food(u32),
}

pub fn parse(raw: &str) -> Vec<Elf> {
	let mut elfs = Vec::new();
	let mut tokens = raw.lines().map(to_token).peekable();

	while tokens.peek().is_some() {
		if let Some(elf) = parse_elf(&mut tokens) {
			elfs.push(elf);
		}
	}

	elfs
}

fn to_token(line: &str) -> Token {
	if let Result::Ok(i) = line.parse::<u32>() {
		Token::Food(i)
	} else if line.is_empty() {
		Token::Empty
	} else {
		panic!("INVALID INPUT")
	}
}

fn parse_elf(tokens: &mut impl Iterator<Item = Token>) -> Option<Elf> {
	let mut foods = Vec::new();
	while let Some(Token::Food(food)) = tokens.next() {
		foods.push(food);
	}

	if !foods.is_empty() {
		Some(Elf::new(foods))
	} else {
		None
	}
}

impl Elf {
	fn new(food: Vec<u32>) -> Self {
		Elf { food }
	}
}

impl PartialEq for Elf {
	fn eq(&self, other: &Self) -> bool {
		self.food == other.food
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn empty_in_empty_out() {
		let input = "";
		let result = parse(input);
		assert_eq!(result, []);
	}

	#[test]
	fn single_food_item() {
		let input = "1000";
		let result = parse(input);
		let expected = [Elf::new(vec![1000])];
		assert_eq!(result, expected);
	}

	#[test]
	fn multiple_elfs_item() {
		let input = "1000\n\n4000\n\n2000";
		let result = parse(input);

		let expected = [
			Elf::new(vec![1000]),
			Elf::new(vec![4000]),
			Elf::new(vec![2000]),
		];

		assert_eq!(result, expected);
	}

	#[test]
	#[should_panic]
	fn invalid_input_space_before_number() {
		// N
		let input = " 1000";
		parse(input);
	}

	#[test]
	fn multiple_elfs_with_multiple_item() {
		let input = "1000\n\n4000\n3000\n\n2000";
		let result = parse(input);

		let expected = [
			Elf::new(vec![1000]),
			Elf::new(vec![4000, 3000]),
			Elf::new(vec![2000]),
		];

		assert_eq!(result, expected);
	}
}
