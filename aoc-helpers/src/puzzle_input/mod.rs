mod web_fetcher;

use std::error::Error;
use std::fs;

use crate::errors::AocError;

pub struct Date {
    day: u8,
    year: u16,
}

impl Date {
    pub fn new(day: u8, year: u16) -> Self {
        Date { day, year }
    }
}

pub fn parse_day(day: Option<String>) -> Result<u8, Box<dyn Error + 'static>> {
    let day_string = day.ok_or(AocError::new("Need to set day"))?;
    let day: u8 = day_string
        .parse()
        .map_err(|e| AocError::from_err("Failed parsing day:", e))?;
    Ok(day)
}

pub fn get_puzzle_input(date: &Date) -> Result<String, Box<dyn Error>> {
    let path = get_path(date);

    if !fs::exists(&path)? {
        initiate_input_data(date)?;
    }

    let content = fs::read_to_string(&path)?;
    Ok(content)
}

fn initiate_input_data(date: &Date) -> Result<(), Box<dyn Error>> {
    let input = web_fetcher::fetch_input(date)?;
    write_to_file(&input, date)
}

fn write_to_file(contents: &str, date: &Date) -> Result<(), Box<dyn Error>> {
    let path = get_dir_path(date);

    fs::DirBuilder::new()
        .recursive(true)
        .create(&path)?;

    let path = get_path(date);
    fs::write(&path, contents)?;
    Ok(())
}

pub fn get_path(date: &Date) -> String {
    format!("{}/input", get_dir_path(date))
}

pub fn get_dir_path(Date { day, year }: &Date) -> String {
    format!("puzzle-input/{year}/{day:02}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_path_single_digit() {
        let path = get_path(&Date::new(3, 2022));
        assert_eq!(path, "puzzle-input/2022/03/input");
    }

    #[test]
    fn get_path_double_digit() {
        let path = get_path(&Date::new(42, 2024));
        assert_eq!(path, "puzzle-input/2024/42/input");
    }
}
