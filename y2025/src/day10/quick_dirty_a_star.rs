use super::{Button, Joltage, LightIndex, Machine, Presses};

use std::{cmp::Ordering, collections::BinaryHeap, ops::Not};

#[allow(dead_code)]
pub fn find_fewest_presses(machine: &Machine) -> Presses {
	let mut search = AStar::new(machine);
	search.find_solution_length()
}

struct AStar<'a> {
	machine: &'a Machine,
	queue: BinaryHeap<ANode>,
}

impl<'a> AStar<'a> {
	fn new(machine: &'a Machine) -> Self {
		let mut res = Self {
			machine,
			queue: BinaryHeap::new(),
		};

		let node = ANode::initial(res.goal());
		res.add_item(node);

		res
	}

	fn find_solution_length(&mut self) -> Presses {
		loop {
			let node = self.queue.pop().unwrap();
			if node.is_state(self.goal()) {
				break node.cost;
			}
			self.add_children(&node);
		}
	}

	fn add_children(&mut self, node: &ANode) {
		let skip = node.start_index;
		for (i, b) in self.machine.buttons.iter().enumerate().skip(skip) {
			let new = node.next(b, &self.machine.joltage_goals, i);
			if new.is_valid(&self.machine.joltage_goals) {
				self.queue.push(new);
			}
		}
	}

	fn add_item(&mut self, node: ANode) {
		self.queue.push(node);
	}

	fn goal(&self) -> &[Joltage] {
		&self.machine.joltage_goals
	}
}

#[derive(PartialEq, Eq, Debug)]
struct ANode {
	joltages: Vec<Joltage>,
	cost: Presses,
	heuristic: Presses,
	start_index: LightIndex,
}

impl ANode {
	fn initial(goal: &[Joltage]) -> Self {
		let joltages = vec![0; goal.len()];
		let heuristic = calculate_heuristic(&joltages, goal);
		Self {
			joltages,
			cost: 0,
			heuristic,
			start_index: 0,
		}
	}

	fn is_state(&self, state: &[Joltage]) -> bool {
		self.joltages
			.iter()
			.zip(state.iter())
			.all(|(own, goal)| own == goal)
	}

	fn is_valid(&self, goal: &[Joltage]) -> bool {
		self.joltages
			.iter()
			.zip(goal.iter())
			.any(|(own, goal)| own > goal)
			.not()
	}

	fn graph_weight(&self) -> Presses {
		self.cost + self.heuristic
	}

	fn next(
		&self,
		button: &Button,
		goal: &[Joltage],
		index: LightIndex,
	) -> Self {
		let joltages = press(&self.joltages, button);
		let cost = &self.cost + 1;
		let heuristic = calculate_heuristic(&joltages, goal);

		Self {
			joltages,
			cost,
			heuristic,
			start_index: index,
		}
	}
}

fn calculate_heuristic(joltages: &[u32], goal: &[u32]) -> u32 {
	joltages
		.iter()
		.zip(goal.iter())
		.map(|(j, g)| (*j).abs_diff(*g))
		.max()
		.unwrap()
}

fn press(joltages: &[Joltage], button: &Button) -> Vec<Joltage> {
	let mut joltages = joltages.to_vec();
	for i in &button.lights {
		joltages[*i] += 1;
	}
	joltages
}

impl Ord for ANode {
	fn cmp(&self, other: &Self) -> Ordering {
		match other.graph_weight().cmp(&self.graph_weight()) {
			Ordering::Equal => self.cost.cmp(&other.cost),
			x => x,
		}
	}
}

impl PartialOrd for ANode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn compare_goal_test() {
		let goal = [1, 2, 3];

		let data = node(vec![1, 2, 3]);
		assert!(data.is_valid(&goal));
		assert!(data.is_state(&goal));

		let data = node(vec![3, 2, 1]);
		assert!(!data.is_valid(&goal));
		assert!(!data.is_state(&goal));

		let data = node(vec![0, 0, 0]);
		assert!(data.is_valid(&goal));
		assert!(!data.is_state(&goal));

		fn node(joltages: Vec<Joltage>) -> ANode {
			ANode {
				joltages,
				..TEMPLATE
			}
		}
	}

	#[test]
	fn next_test() {
		let goal = [1, 2, 3];
		let node = ANode::initial(&goal);
		assert_eq!(node.graph_weight(), 3);

		let node = node.next(&button(vec![0, 1]), &goal, 0);
		assert_eq!(node.cost, 1);
		assert_eq!(node.graph_weight(), 4);
		assert!(!node.is_state(&goal));

		let node = node.next(&button(vec![2]), &goal, 0);
		assert_eq!(node.cost, 2);
		assert_eq!(node.graph_weight(), 4);
		assert!(!node.is_state(&goal));

		let node = node.next(&button(vec![2]), &goal, 0);
		assert_eq!(node.cost, 3);
		assert_eq!(node.graph_weight(), 4);
		assert!(!node.is_state(&goal));

		let node = node.next(&button(vec![2]), &goal, 0);
		assert_eq!(node.cost, 4);
		assert_eq!(node.graph_weight(), 5);
		assert!(!node.is_state(&goal));

		let node = node.next(&button(vec![1]), &goal, 0);
		assert_eq!(node.cost, 5);
		assert_eq!(node.graph_weight(), 5);
		assert!(node.is_state(&goal));
	}

	#[test]
	fn node_sorting() {
		let items = [
			node(1, 0),
			node(3, 1),
			node(1, 2),
			node(1, 4),
			node(2, 3),
		];

		let mut items: BinaryHeap<_> = items.into_iter().collect();

		assert_eq!(items.pop(), Some(node(1, 0)));
		assert_eq!(items.pop(), Some(node(1, 2)));
		assert_eq!(items.pop(), Some(node(3, 1)));
		assert_eq!(items.pop(), Some(node(2, 3)));
		assert_eq!(items.pop(), Some(node(1, 4)));

		fn node(cost: Presses, heuristic: Presses) -> ANode {
			ANode {
				cost,
				heuristic,
				..TEMPLATE
			}
		}
	}

	fn button(lights: Vec<LightIndex>) -> Button {
		Button { lights }
	}

	const TEMPLATE: ANode = ANode {
		cost: 0,
		heuristic: 0,
		joltages: vec![],
		start_index: 0,
	};
}
