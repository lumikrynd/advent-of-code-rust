use aoc_helpers::errors::AocError;
use aoc_helpers::{PuzzleSolver, parse_day};
use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn solve_day(day: Option<String>) -> Result<String, Box<dyn Error>> {
    let day = parse_day(day)?;
    let input = get_puzzle_input(day)?;

    let solver = get_solver(day, &input)?;
    let solution = solver.solve();
    Ok(solution)
}

fn get_solver(day: u8, input: &str) -> Result<Box<dyn PuzzleSolver>, Box<dyn Error>> {
    let solver: Box<dyn PuzzleSolver> = match day {
        1 => day1::Solver::new(input),
        2 => day2::Solver::new(input),
        3 => day3::Solver::new(input),
        4 => day4::Solver::new(input),
        x => Err(AocError::new(&format!("No solver for day {x}")))?,
    };
    Ok(solver)
}

fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
    let date = aoc_helpers::Date::new(day, 2022);
    aoc_helpers::get_puzzle_input(&date)
}
