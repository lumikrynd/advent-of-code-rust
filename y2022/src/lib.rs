use aoc_helpers::errors::AocError;
use std::error::Error;

mod day1;

pub fn solve_day(
    day: Option<String>,
    _test_data: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let day = parse_day(day)?;
    let input = get_puzzle_input(day)?;

    match day {
        1 => Ok(day1::solve(&input)),
        _ => Err(Box::new(AocError::new("Not Implemented"))),
    }
}

fn parse_day(day: Option<String>) -> Result<u8, Box<dyn Error + 'static>> {
    let day_string = day.ok_or(AocError::new("Need to set day"))?;
    let day: u8 = day_string
        .parse()
        .map_err(|e| AocError::from_err("Failed parsing day:", e))?;
    Ok(day)
}

fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
    let date = aoc_helpers::Date::new(day, 2022);
    aoc_helpers::get_puzzle_input(&date)
}
