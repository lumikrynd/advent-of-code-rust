use std::cmp::Reverse;
use std::collections::BinaryHeap;

use graph::*;

use aoc_helpers::{PuzzleSolver, combinations_set};

mod graph;

type Point = aoc_helpers::Point3D;

#[derive(Debug, PartialEq)]
pub struct Solver {
	boxes: Vec<Point>,
	take: usize,
}

impl Solver {
	pub fn new(input: &str) -> Box<Self> {
		let s = Solver {
			boxes: parse(input),
			take: 1000,
		};
		Box::new(s)
	}
}

impl PuzzleSolver for Solver {
	fn solve_part_1(&self) -> Option<String> {
		let connnections = find_closest(&self.boxes, self.take);
		let graphs = create_graph(connnections);

		let mut sizes: Vec<_> = graphs
			.groups()
			.iter()
			.filter(|g| !g.is_empty())
			.map(|g| g.len())
			.collect();

		sizes.sort_unstable();

		let res = sizes
			.into_iter()
			.rev()
			.take(3)
			.reduce(|a, b| a * b)
			.unwrap();

		Some(res.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		let target_len = self.boxes.len();

		let mut connections = find_closest_all(&self.boxes);
		let mut graph = Graph::new();

		let last = loop {
			let (a, b) = connections.next().expect("");
			graph.connect(a, b);

			let groups = graph.groups();
			if groups.len() == 1 && groups[0].len() == target_len {
				break (a, b);
			}
		};

		let result = i64::from(last.0.x()) * i64::from(last.1.x());
		Some(result.to_string())
	}
}

fn create_graph(connnections: Vec<(Point, Point)>) -> Graph {
	let mut graph = Graph::new();

	for (a, b) in connnections {
		graph.connect(a, b);
	}

	graph
}

/// Brute force implementation
fn find_closest(points: &[Point], count: usize) -> Vec<(Point, Point)> {
	let mut combinations = combinations_set(points.iter());

	let mut heap = BinaryHeap::new();
	for (a, b) in combinations.by_ref().take(count) {
		let item = HeapItem::from(*a, *b);
		heap.push(item);
	}

	for (a, b) in combinations {
		let new = HeapItem::from(*a, *b);
		heap.push(new);
		heap.pop();
	}

	let temp: Vec<_> = std::iter::from_fn(move || heap.pop())
		.map(|HeapItem(_, p)| p)
		.map(fix_order)
		.collect();
	temp.into_iter().rev().collect()
}

/// Brute force implementation
/// Wonder if this can even run...
fn find_closest_all(points: &[Point]) -> impl Iterator<Item = (Point, Point)> {
	let mut combinations = combinations_set(points.iter());

	let mut heap = BinaryHeap::new();
	for (a, b) in combinations.by_ref() {
		let item = HeapItem::from(*a, *b);
		heap.push(Reverse(item));
	}

	std::iter::from_fn(move || heap.pop())
		.map(|r| r.0)
		.map(|HeapItem(_, p)| p)
		.map(fix_order)
}

#[derive(PartialEq)]
struct HeapItem(f64, (Point, Point));

impl HeapItem {
	fn from(a: Point, b: Point) -> Self {
		let dist = a.distance(&b);
		HeapItem(dist, (a, b))
	}
}

impl PartialOrd for HeapItem {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for HeapItem {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.partial_cmp(&other.0).unwrap()
	}
}

impl Eq for HeapItem {}

fn fix_order((a, b): (Point, Point)) -> (Point, Point) {
	match a.cmp(&b) {
		std::cmp::Ordering::Less => (a, b),
		std::cmp::Ordering::Equal => panic!("same"),
		std::cmp::Ordering::Greater => (b, a),
	}
}

fn parse(input: &str) -> Vec<Point> {
	input
		.lines()
		.map(|l| {
			let (a, l) = l.split_once(',').unwrap();
			let (b, c) = l.split_once(',').unwrap();

			let (a, b, c) =
				(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap());
			Point::new(a, b, c)
		})
		.collect()
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

	#[test]
	fn part_1_test() {
		let s = Solver {
			boxes: parse(EXAMPLE),
			take: 10,
		};

		assert_eq!(s.solve_part_1().expect("None"), "40");
	}

	#[test]
	fn part_2_test() {
		let s = Solver {
			boxes: parse(EXAMPLE),
			take: 10,
		};

		assert_eq!(s.solve_part_2().expect("None"), "25272");
	}

	#[test]
	fn find_closest_test() {
		let points = [
			point(0, 0, 0),
			point(1, 0, 0),
			point(10, 0, 0),
			point(12, 0, 0),
		];

		let result = find_closest(&points, 1);
		assert_eq!(result, [(point(0, 0, 0), point(1, 0, 0))]);

		let result = find_closest(&points, 3);
		assert_eq!(
			result,
			[
				(point(0, 0, 0), point(1, 0, 0)),
				(point(10, 0, 0), point(12, 0, 0)),
				(point(1, 0, 0), point(10, 0, 0)),
			]
		);
	}

	#[test]
	fn parse_test() {
		let res = parse(EXAMPLE);

		assert_eq!(res[0], Point::new(162, 817, 812), "first");
		assert_eq!(res.len(), 20, "length");
		let last = res.iter().last().unwrap();
		assert_eq!(*last, Point::new(425, 690, 689), "last");
	}

	fn point(x: i32, y: i32, z: i32) -> Point {
		Point::new(x, y, z)
	}

	const EXAMPLE: &str = indoc! {"
		162,817,812
		57,618,57
		906,360,560
		592,479,940
		352,342,300
		466,668,158
		542,29,236
		431,825,988
		739,650,466
		52,470,668
		216,146,977
		819,987,18
		117,168,530
		805,96,715
		346,949,466
		970,615,88
		941,993,340
		862,61,35
		984,92,344
		425,690,689"};
}
