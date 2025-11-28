use aoc_helpers::PuzzleSolver;

mod map;
use map::Map;

pub struct Solver {
	map: Map,
}

impl PuzzleSolver for Solver {}

impl Solver {
	pub fn new(input: &str) -> Box<Solver> {
		let s = Solver {
			map: Map::new(input),
		};
		Box::new(s)
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn name() {}
}
