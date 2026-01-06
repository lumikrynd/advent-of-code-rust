use aoc_helpers::PuzzleSolver;
use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;

mod parsing;

type Count = i64;

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
		let result = self.path_count("you", "out");
		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let (start, end, mut mid_1, mut mid_2) = ("svr", "out", "dac", "fft");
		let mut mid_count = self.path_count(mid_1, mid_2);

		if mid_count == 0 {
			(mid_1, mid_2) = (mid_2, mid_1);
			mid_count = self.path_count(mid_1, mid_2);
		}

		let start_count = self.path_count(start, mid_1);
		let end_count = self.path_count(mid_2, end);

		let result = start_count * mid_count * end_count;
		Some(result.to_string())
	}
}

#[derive(Debug, PartialEq)]
struct Server<'a> {
	name: &'a str,
	out: Vec<&'a str>,
}

impl<'a> Solver<'a> {
	fn path_count(&self, from: &'a str, to: &'a str) -> Count {
		let out_map = output_map(&self.servers);
		let mut in_map = input_map(&self.servers);

		let mut ready: Vec<_> = in_map
			.iter()
			.filter_map(|(key, ins)| ins.is_empty().then_some(*key))
			.collect();

		let mut counter = path_counter::PathCounter::new(from);

		while let Some(server) = ready.pop() {
			for out in out_map[server].iter() {
				counter.connect(server, out);
				let removed = in_map.entry(out).or_default().remove(server);
				assert!(removed);

				if in_map[out].is_empty() {
					ready.push(out);
				}
			}
		}

		counter.get(to)
	}
}

///Responsible for keeping count of all paths from one points to any other point
///Note that "connect" assumes the count of "from" is done.
mod path_counter {
	use super::*;

	pub struct PathCounter<'a> {
		counts: HashMap<&'a str, Count>,
	}

	impl<'a> PathCounter<'a> {
		pub fn new(start: &'a str) -> PathCounter<'a> {
			let mut counts = HashMap::new();
			counts.insert(start, 1);
			PathCounter { counts }
		}

		pub fn connect(&mut self, from: &'a str, to: &'a str) {
			let from_count = *self.counts.entry(from).or_default();
			self.counts.entry(to).or_default().add_assign(from_count);
		}

		pub fn get(&self, name: &'a str) -> Count {
			self.counts.get(name).cloned().unwrap_or_default()
		}
	}
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
	fn part_2() {
		let solver = Solver::new(EXAMPLE_2);
		assert_eq!(solver.solve_part_2().unwrap_or_default(), "2")
	}

	#[test]
	fn input_map_test() {
		let input = vec![
			Server("AAA", vec!["BBB", "CCC", "DDD"]),
			Server("BBB", vec!["DDD"]),
		];

		let result = input_map(&input);

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

	const EXAMPLE_2: &str = indoc! {"
		svr: aaa bbb
		aaa: fft
		fft: ccc
		bbb: tty
		tty: ccc
		ccc: ddd eee
		ddd: hub
		hub: fff
		eee: dac
		dac: fff
		fff: ggg hhh
		ggg: out
		hhh: out"};
}
