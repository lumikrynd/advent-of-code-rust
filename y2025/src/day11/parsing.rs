use super::*;

pub fn parse<'a>(input: &'a str) -> Vec<Server<'a>> {
	input.lines().map(parse_line).collect()
}

fn parse_line<'a>(input: &'a str) -> Server<'a> {
	let (name, out) = input.split_once(": ").unwrap();
	let out = out.split(' ').collect();
	Server { name, out }
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn parsing() {
		let input = indoc! {"
			aaa: you hhh
			you: bbb ccc"};

		let expected = vec![
			Server {
				name: "aaa",
				out: vec!["you", "hhh"],
			},
			Server {
				name: "you",
				out: vec!["bbb", "ccc"],
			},
		];

		assert_eq!(parse(input), expected);
	}
}
