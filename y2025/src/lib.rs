use aoc_helpers::errors::AocError;
use aoc_helpers::PuzzleSolver;
use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

pub fn solve_day(
	mut arguments: impl Iterator<Item = String>,
) -> Result<String, Box<dyn Error>> {
	let day = arguments.next().expect("Need day argument");
	let day = day.parse().expect("Invalid day value");
	let input = get_puzzle_input(day)?;

	let solver = get_solver(day, &input)?;
	let solution = solver.solve();
	Ok(solution)
}

fn get_solver<'a>(
	day: u8,
	input: &'a str,
) -> Result<Box<dyn PuzzleSolver + 'a>, Box<dyn Error>> {
	let solver: Box<dyn PuzzleSolver> = match day {
		1 => day1::Solver::new(input),
		2 => day2::Solver::new(input),
		3 => day3::Solver::new(input),
		4 => day4::Solver::new(input),
		5 => day5::Solver::new(input),
		6 => day6::Solver::new(input),
		7 => day7::Solver::new(input),
		8 => day8::Solver::new(input),
		9 => day9::Solver::new(input),
		10 => day10::Solver::new(input),
		11 => day11::Solver::new(input),
		x => Err(AocError::new(&format!("No solver for day {x}")))?,
	};
	Ok(solver)
}

fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
	let date = aoc_helpers::Date::new(day, 2025);
	aoc_helpers::get_puzzle_input(&date)
}
