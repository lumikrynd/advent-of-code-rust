mod web_fetcher;

use std::error::Error;
use std::fs;

pub fn get_puzzle_input(day: u8, year: u16) -> Result<String, Box<dyn Error>> {
    let path = get_path(day, year);

    if !fs::exists(&path)? {
        initiate_input_data(day, year)?;
    }

    let content = fs::read_to_string(&path)?;
    Ok(content)
}

fn initiate_input_data(day: u8, year: u16) -> Result<(), Box<dyn Error>> {
    let input = web_fetcher::fetch_input(day, year)?;
    write_to_file(&input, day, year)
}

fn write_to_file(contents: &str, day: u8, year: u16) -> Result<(), Box<dyn Error>> {
    let path = get_dir_path(day, year);

    fs::DirBuilder::new()
        .recursive(true)
        .create(&path)?;

    let path = get_path(day, year);
    fs::write(&path, contents)?;
    Ok(())
}

pub fn get_path(day: u8, year: u16) -> String {
    format!("{}/input", get_dir_path(day, year))
}

pub fn get_dir_path(day: u8, year: u16) -> String {
    format!("puzzle-input/{year}/{day:02}")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_path_single_digit() {
        let path = get_path(3, 2022);
        assert_eq!(path, "puzzle-input/2022/03/input");
    }

    #[test]
    fn get_path_double_digit() {
        let path = get_path(42, 2024);
        assert_eq!(path, "puzzle-input/2024/42/input");
    }
}

