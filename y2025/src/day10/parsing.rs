use super::{Button, Joltage, Machine};

pub fn parse(input: &str) -> Vec<Machine> {
	input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Machine {
	let (light_goal, rest) = input.split_once(' ').unwrap();
	let (buttons, light_joltage) = rest.rsplit_once(' ').unwrap();

	Machine {
		light_goals: parse_light_goals(light_goal),
		buttons: parse_buttons(buttons),
		joltage_goals: parse_joltage(light_joltage),
	}
}

fn parse_light_goals(light_goal: &str) -> Vec<bool> {
	let light_goal = light_goal.trim_matches(['[', ']']);
	light_goal.chars().map(parse_light_goal).collect()
}

fn parse_light_goal(s: char) -> bool {
	match s {
		'#' => true,
		'.' => false,
		_ => panic!("What is going on: '{s}'"),
	}
}

fn parse_buttons(buttons: &str) -> Vec<Button> {
	buttons.split(' ').map(parse_button).collect()
}

fn parse_button(button: &str) -> Button {
	let button = button.trim_matches(['(', ')']);
	let lights = button.split(',').map(|l| l.parse().unwrap()).collect();
	Button { lights }
}

fn parse_joltage(joltage: &str) -> Vec<Joltage> {
	let joltage = joltage.trim_matches(['{', '}']);
	joltage.split(',').map(|j| j.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::super::{Joltage, LightIndex};
	use super::*;

	#[test]
	fn parse_test() {
		let input = indoc! {"
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"};

		let result = parse(input);
		let expected = vec![
			Machine(
				vec![false, true, true, false],
				vec![
					Button(vec![3]),
					Button(vec![1, 3]),
					Button(vec![2]),
					Button(vec![2, 3]),
					Button(vec![0, 2]),
					Button(vec![0, 1]),
				],
				vec![3, 5, 4, 7],
			),
			Machine(
				vec![false, false, false, true, false],
				vec![
					Button(vec![0, 2, 3, 4]),
					Button(vec![2, 3]),
					Button(vec![0, 4]),
					Button(vec![0, 1, 2]),
					Button(vec![1, 2, 3, 4]),
				],
				vec![7, 5, 12, 7, 2],
			),
		];

		assert_eq!(result, expected);
	}

	#[allow(non_snake_case)]
	fn Machine(
		light_goals: Vec<bool>,
		buttons: Vec<Button>,
		joltage_goals: Vec<Joltage>,
	) -> Machine {
		Machine {
			light_goals,
			buttons,
			joltage_goals,
		}
	}

	#[allow(non_snake_case)]
	fn Button(lights: Vec<LightIndex>) -> Button {
		Button { lights }
	}
}
