use super::{Machine, Presses};

use good_lp::{
	Expression, ProblemVariables, Solution, SolverModel, constraint,
	default_solver, variable,
};

#[allow(dead_code)]
pub fn find_fewest_presses(machine: &Machine) -> Presses {
	let mut variables = ProblemVariables::new();

	let buttons: Vec<_> = machine
		.buttons
		.iter()
		.map(|b| (b, variables.add(variable().integer().min(0))))
		.collect();

	let objective: Expression = buttons.iter().fold(0.into(), |a, b| a + b.1);

	let mut problem = variables
		.minimise(objective.clone())
		.using(default_solver);

		problem.set_parameter("loglevel", "0");

	for (i, goal) in machine.joltage_goals.iter().enumerate() {
		let exp: Expression = buttons
			.iter()
			.filter(|b| b.0.lights.contains(&i))
			.fold(0.into(), |a, b| a + b.1);

		problem.add_constraint(constraint!(exp == *goal));
	}

	// Solve
	let solution = problem.solve().expect("Welp, let's hope it works");
	let res = solution.eval(&objective);

	res as Presses
}

#[cfg(test)]
mod test {
	use super::*;

	use super::super::Button;
	use super::super::{Joltage, LightIndex, Machine};

	#[test]
	fn single_button() {
		let input = Machine(vec![Button(vec![0])], vec![10]);
		assert_eq!(find_fewest_presses(&input), 10);
	}

	#[test]
	fn multi_buttons() {
		let input =
			Machine(vec![Button(vec![0]), Button(vec![1])], vec![10, 5]);
		assert_eq!(find_fewest_presses(&input), 15);
	}

	#[test]
	fn overlapping_buttons() {
		let input =
			Machine(vec![Button(vec![0, 1]), Button(vec![0])], vec![10, 5]);
		assert_eq!(find_fewest_presses(&input), 10);
	}

	#[allow(non_snake_case)]
	fn Machine(buttons: Vec<Button>, joltage_goals: Vec<Joltage>) -> Machine {
		Machine {
			light_goals: vec![],
			buttons,
			joltage_goals,
		}
	}

	#[allow(non_snake_case)]
	fn Button(lights: Vec<LightIndex>) -> Button {
		Button { lights }
	}
}
