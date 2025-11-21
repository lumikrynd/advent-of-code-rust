use aoc_helpers::errors::AocError;
use std::error::Error;

mod day1;

pub fn solve_day(
    day: Option<String>,
    _test_data: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let day = day.ok_or(AocError::new("Need to set day"))?;
    let day: u8 = day
        .parse()
        .map_err(|e| AocError::new(&format!("Failed parsing day: {}", e)))?;

    let input = get_puzzle_input(day)?;

    match day {
        1 => Ok(day1::solve(&input)),
        _ => Err(Box::new(AocError::new("Not Implemented"))),
    }
}

fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
    let date = aoc_helpers::Date::new(day, 2022);
    aoc_helpers::get_puzzle_input(&date)
}
