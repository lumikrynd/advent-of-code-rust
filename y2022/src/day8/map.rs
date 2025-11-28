pub struct Map {
	content: Vec<u8>,
	columns: usize,
	rows: usize,
}

impl Map {
	pub fn new(input: &str) -> Map {
		let mut lines = input.lines();
		let first = lines.next().expect("Empty input");
		let mut map = Self::init(first);

		for row in lines {
			map.add_row(row);
		}

		map
	}

	pub fn get(&self, x: usize, y: usize) -> &u8 {
		let i = (y*self.columns) + x;
		&self.content[i]
	}

	fn init(first_line: &str) -> Map {
		let mut map = Map {
			content: vec![],
			columns: first_line.chars().count(),
			rows: 0,
		};

		map.add_row(first_line);
		map
	}

	fn add_row(&mut self, row: &str){
		if row.chars().count() != self.columns {
			panic!("Badly formed input")
		}

		self.rows += 1;

		for c in row.chars() {
			let digit = Self::to_digit(c);
			self.content.push(digit);
		}
	}

	fn to_digit(c: char) -> u8 {
		c.to_digit(10).unwrap().try_into().unwrap()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn valid_index() {
		let input = indoc! {"
			0123
			4567
			8910"};

		let map = Map::new(input);

		assert_eq!(7, *map.get(3, 1));
		assert_eq!(8, *map.get(0, 2));
		assert_eq!(0, *map.get(0, 0));
		assert_eq!(0, *map.get(3, 2));
	}

	#[test]
	#[should_panic]
	fn invalid_new_1() {
		let input = indoc! {"
			1234
			12345"};

		let _ = Map::new(input);
	}

	#[test]
	#[should_panic]
	fn invalid_new_2() {
		let input = indoc! {"
			12345
			1234"};

		let _ = Map::new(input);
	}
}
