use isahc::{Request, prelude::*};
use std::time::Duration;
use crate::errors::AocError;
use std::error::Error;
use std::fs;

pub fn fetch_input(day: u8, year: u16) -> Result<String, Box<dyn Error + 'static>> {
    let url = get_url(day, year);
    let cookie = fs::read_to_string("puzzle-input/cookie")
        .map_err(|err| AocError::from_err("Failed to get cookie", err))?
        .trim().to_string();

    let mut response = Request::post(&url)
        .header("Cookie", &cookie)
        .timeout(Duration::from_secs(5))
        .body(())?
        .send()?;

    let body = response.text()?;

    if let 200 = response.status().as_u16() {
        Ok(body)
    } else {
        let err_msg = format!("Unexpected response from '{url}'\n{body}");
        Err(AocError::new(&err_msg))?
    }
}

pub fn get_url(day: u8, year: u16) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_url_single_digit() {
        let url = get_url(3, 2022);
        assert_eq!(url, "https://adventofcode.com/2022/day/3/input");
    }

    #[test]
    fn get_url_double_digit() {
        let url = get_url(42, 2024);
        assert_eq!(url, "https://adventofcode.com/2024/day/42/input");
    }
}

