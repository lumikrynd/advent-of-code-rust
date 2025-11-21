use crate::errors::AocError;
use std::{error::Error, u8};

mod day1;
mod errors;

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
    aoc_helpers::get_puzzle_input(day, 2022)
}
