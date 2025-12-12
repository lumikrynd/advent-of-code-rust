use super::{Button, Light, Machine};

pub fn parse(input: &str) -> Vec<Machine> {
	input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Machine {
	let (light_goal, rest) = input.split_once(' ').unwrap();
	let (buttons, light_joltage) = rest.rsplit_once(' ').unwrap();

	Machine {
		lights: parse_lights(light_goal, light_joltage),
		buttons: parse_buttons(buttons),
	}
}

fn parse_lights(light_goal: &str, light_joltage: &str) -> Vec<Light> {
	let light_goal = light_goal.trim_matches(['[', ']']);
	let light_joltage = light_joltage.trim_matches(['{', '}']);

	light_goal
		.chars()
		.zip(light_joltage.split(','))
		.map(|(goal, jolt)| (parse_light_goal(goal), jolt.parse().unwrap()))
		.map(|(should_be_on, joltage)| Light {
			should_be_on,
			joltage,
		})
		.collect()
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

	let lights = button
		.split(',')
		.map(|l| l.parse().unwrap())
		.collect();

	Button { lights }
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
				vec![
					Light(false, 3),
					Light(true, 5),
					Light(true, 4),
					Light(false, 7),
				],
				vec![
					Button(vec![3]),
					Button(vec![1, 3]),
					Button(vec![2]),
					Button(vec![2, 3]),
					Button(vec![0, 2]),
					Button(vec![0, 1]),
				],
			),
			Machine(
				vec![
					Light(false, 7),
					Light(false, 5),
					Light(false, 12),
					Light(true, 7),
					Light(false, 2),
				],
				vec![
					Button(vec![0, 2, 3, 4]),
					Button(vec![2, 3]),
					Button(vec![0, 4]),
					Button(vec![0, 1, 2]),
					Button(vec![1, 2, 3, 4]),
				],
			),
		];

		assert_eq!(result, expected);
	}

	#[allow(non_snake_case)]
	fn Machine(lights: Vec<Light>, buttons: Vec<Button>) -> Machine {
		Machine { lights, buttons }
	}

	#[allow(non_snake_case)]
	fn Light(should_be_on: bool, joltage: Joltage) -> Light {
		Light {
			should_be_on,
			joltage,
		}
	}

	#[allow(non_snake_case)]
	fn Button(lights: Vec<LightIndex>) -> Button {
		Button { lights }
	}
}
