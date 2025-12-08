use super::*;

use std::collections::{HashMap, HashSet};

pub struct Graph {
	groups: Vec<HashSet<Point>>,
	members: HashMap<Point, usize>,
}

impl Graph {
	pub fn new() -> Graph {
		Graph {
			groups: vec![],
			members: HashMap::new(),
		}
	}

	pub fn groups(&self) -> Vec<&HashSet<Point>> {
		self.groups.iter().filter(|g| !g.is_empty()).collect()
	}

	pub fn new_group(&mut self) -> usize {
		let g_id = self.groups.len();
		self.groups.push(HashSet::new());
		g_id
	}

	pub fn insert(&mut self, g_id: usize, point: Point) {
		self.groups[g_id].insert(point);
		self.members.insert(point, g_id);
	}

	pub fn merge(&mut self, a: usize, b: usize) {
		if a == b {
			return;
		}

		self.new_group();
		let b_group = self.groups.swap_remove(b);

		for mem in b_group {
			self.insert(a, mem);
		}
	}

	pub fn connect(&mut self, a: Point, b: Point) {
		let graph_a = self.members.get(&a);
		let graph_b = self.members.get(&b);
		match (graph_a, graph_b) {
			(None, None) => {
				let g_id = self.new_group();
				self.insert(g_id, a);
				self.insert(g_id, b);
			}
			(Some(g_id), None) | (None, Some(g_id)) => {
				let g_id = g_id.to_owned();
				self.insert(g_id, a);
				self.insert(g_id, b);
			}
			(Some(a_gid), Some(b_gid)) => {
				let (a_gid, b_gid) = (a_gid.to_owned(), b_gid.to_owned());
				self.merge(a_gid, b_gid);
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn insert_test() {
		let mut graph = Graph::new();
		let g_id = graph.new_group();

		let a = Point::new(0, 0, 0);
		let b = Point::new(1, 0, 0);
		graph.insert(g_id, a);
		graph.insert(g_id, b);

		let group = graph.get_group(g_id);
		assert_eq!(group.len(), 2);
		assert!(group.contains(&a));
		assert!(group.contains(&b));

		let mem = graph.members();
		assert_eq!(mem.len(), 2);
		assert!(mem.contains_key(&a));
		assert!(mem.contains_key(&b));
	}

	#[test]
	fn merge_test() {
		let a = Point::new(0, 0, 0);
		let b = Point::new(1, 0, 0);
		let c = Point::new(2, 0, 0);
		let d = Point::new(3, 0, 0);
		let e = Point::new(4, 0, 0);
		let f = Point::new(5, 0, 0);

		let mut graph = Graph::new();
		let g_a = graph.new_group();
		let g_b = graph.new_group();

		graph.insert(g_a, a);
		graph.insert(g_a, b);
		graph.insert(g_a, c);
		graph.insert(g_a, d);

		graph.insert(g_b, e);
		graph.insert(g_b, f);

		graph.merge(g_a, g_b);

		let mut group = graph.get_group(g_a);
		let mut g_id = g_a;
		if group.is_empty() {
			group = graph.get_group(g_b);
			g_id = g_b;
		}

		assert_eq!(group.len(), 6);
		assert!(group.contains(&a));
		assert!(group.contains(&b));
		assert!(group.contains(&c));
		assert!(group.contains(&d));
		assert!(group.contains(&e));
		assert!(group.contains(&f));

		let mem = graph.members();
		assert_eq!(mem[&a], g_id);
		assert_eq!(mem[&b], g_id);
		assert_eq!(mem[&c], g_id);
		assert_eq!(mem[&d], g_id);
		assert_eq!(mem[&e], g_id);
		assert_eq!(mem[&f], g_id);

		graph.connect(a, b);
		assert_eq!(graph.groups().len(), 1);
	}

	impl Graph {
		fn get_group(&self, id: usize) -> HashSet<Point> {
			self.groups[id].clone()
		}

		fn members(&self) -> &HashMap<Point, usize> {
			&self.members
		}
	}
}
