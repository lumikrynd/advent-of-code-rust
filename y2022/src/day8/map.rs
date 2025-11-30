pub struct Map {
	content: Vec<Vec<u8>>,
}

impl Map {
	pub fn new(input: &str) -> Map {
		let content = input.lines().map(to_vec).collect();
		let map = Map { content };
		map.panic_if_invalid();

		map
	}

	pub fn get(&self, x: usize, y: usize) -> u8 {
		self.content[y][x]
	}

	pub fn get_dimmensions(&self) -> (usize, usize) {
		let x_len = self.content[0].len();
		let y_len = self.content.len();

		return (x_len, y_len);
	}

	fn panic_if_invalid(&self) {
		let row_length = self.content[0].len();

		if self.content.iter().any(|r| r.len() != row_length) {
			panic!("Map isn't properly rectangular")
		}
	}
}

fn to_vec(line: &str) -> Vec<u8> {
	line.chars().map(to_digit).collect()
}

fn to_digit(c: char) -> u8 {
	c.to_digit(10).unwrap().try_into().unwrap()
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

		assert_eq!(7, map.get(3, 1));
		assert_eq!(8, map.get(0, 2));
		assert_eq!(0, map.get(0, 0));
		assert_eq!(0, map.get(3, 2));
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
