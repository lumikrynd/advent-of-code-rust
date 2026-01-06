use std::{
	collections::{HashMap, HashSet},
	ops::AddAssign,
};

use aoc_helpers::PuzzleSolver;

mod parsing;

pub struct Solver<'a> {
	servers: Vec<Server<'a>>,
}

impl<'a> Solver<'a> {
	pub fn new(input: &'a str) -> Box<Self> {
		let s = Solver {
			servers: parsing::parse(input),
		};
		Box::new(s)
	}
}

impl<'a> PuzzleSolver for Solver<'a> {
	fn solve_part_1(&self) -> Option<String> {
		let out_map = output_map(&self.servers);
		let mut in_map = input_map(&self.servers);

		let mut ready: Vec<_> = in_map
			.iter()
			.filter_map(|(key, ins)| ins.is_empty().then_some(key))
			.cloned()
			.collect();

		let mut values: HashMap<&'a str, i32> = HashMap::new();
		values.insert("you", 1);

		while let Some(server) = ready.pop() {
			let server_val = values.entry(server).or_default().clone();
			for out in out_map[server].iter() {
				values.entry(out).or_default().add_assign(server_val);

				let removed = in_map.entry(out).or_default().remove(server);
				assert!(removed);

				if in_map[out].is_empty() {
					ready.push(out);
				}
			}
		}

		let result = values["out"];
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

#[derive(Debug, PartialEq)]
struct Server<'a> {
	name: &'a str,
	out: Vec<&'a str>,
}

fn output_map<'a>(
	servers: &Vec<Server<'a>>,
) -> HashMap<&'a str, HashSet<&'a str>> {
	let mut map: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();

	for server in servers {
		map.entry(server.name)
			.or_default()
			.extend(server.out.iter());
		for out in server.out.iter() {
			map.entry(out).or_default();
		}
	}

	map
}

fn input_map<'a>(
	servers: &Vec<Server<'a>>,
) -> HashMap<&'a str, HashSet<&'a str>> {
	let mut map: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();

	for server in servers {
		map.entry(server.name).or_default();
		for out in server.out.iter() {
			map.entry(out).or_default().insert(server.name);
		}
	}

	map
}

#[cfg(test)]
mod test {
	use super::*;
	use indoc::indoc;

	#[test]
	fn part_1() {
		let solver = Solver::new(EXAMPLE);
		assert_eq!(solver.solve_part_1().unwrap_or_default(), "5")
	}

	#[test]
	fn input_map_test() {
		let input = vec![
			Server("AAA", vec!["BBB", "CCC", "DDD"]),
			Server("BBB", vec!["DDD"]),
		];

		let result = input_map(&input);

		let empty: Vec<&str> = vec![];
		assert_eq!(result["AAA"], HashSet::from([]));
		assert_eq!(result["BBB"], HashSet::from(["AAA"]));
		assert_eq!(result["CCC"], HashSet::from(["AAA"]));
		assert_eq!(result["DDD"], HashSet::from(["AAA", "BBB"]));
		assert_eq!(result.len(), 4);
	}

	#[test]
	fn output_map_test() {
		let input = vec![
			Server("AAA", vec!["BBB", "CCC", "DDD"]),
			Server("BBB", vec!["DDD"]),
		];

		let result = output_map(&input);

		assert_eq!(result["AAA"], HashSet::from(["BBB", "CCC", "DDD"]));
		assert_eq!(result["BBB"], HashSet::from(["DDD"]));
		assert_eq!(result["CCC"], HashSet::from([]));
		assert_eq!(result["DDD"], HashSet::from([]));
		assert_eq!(result.len(), 4);
	}

	#[allow(non_snake_case)]
	fn Server<'a>(name: &'a str, out: Vec<&'a str>) -> Server<'a> {
		Server { name, out }
	}

	const EXAMPLE: &str = indoc! {"
		aaa: you hhh
		you: bbb ccc
		bbb: ddd eee
		ccc: ddd eee fff
		ddd: ggg
		eee: out
		fff: out
		ggg: out
		hhh: ccc fff iii
		iii: out"};
}
