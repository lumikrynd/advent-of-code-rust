pub trait PuzzleSolver {
	fn solve_part_1(&self) -> Option<String> {
		None
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

fn fallback(item: Option<String>) -> String {
	item.unwrap_or("Not implemented".to_string())
}

impl<'a> dyn PuzzleSolver + 'a {
	pub fn solve(&self) -> String {
		let part_1 = fallback(self.solve_part_1());
		let part_2 = fallback(self.solve_part_2());
		let solution = format!("part1: {}\npart2: {}", part_1, part_2);
		solution
	}
}
