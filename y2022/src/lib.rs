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
    let path = get_path(day);
    let content = std::fs::read_to_string(&path)?;
    Ok(content)
}

fn get_path(day: u8) -> String {
    format!("puzzle-input/2022/{:02}/input", day)
}

#[cfg(test)]
mod test {
    use crate::get_path;

    #[test]
    fn fmt_single_digit() {
        let path = get_path(3);
        assert_eq!(path, "puzzle-input/2022/03/input");
    }

    #[test]
    fn fmt_double_digit() {
        let path = get_path(42);
        assert_eq!(path, "puzzle-input/2022/42/input");
    }
}
