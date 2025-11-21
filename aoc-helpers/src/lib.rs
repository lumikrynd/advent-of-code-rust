use std::error::Error;

pub fn get_puzzle_input(day: u8, year: u16) -> Result<String, Box<dyn Error>> {
    let path = get_path(day, year);
    let content = std::fs::read_to_string(&path)?;
    Ok(content)
}

pub fn get_path(day: u8, year: u16) -> String {
    format!("puzzle-input/{year}/{day:02}/input")
}

#[cfg(test)]
mod test {
    use crate::get_path;

    #[test]
    fn fmt_single_digit() {
        let path = get_path(3, 2022);
        assert_eq!(path, "puzzle-input/2022/03/input");
    }

    #[test]
    fn fmt_double_digit() {
        let path = get_path(42, 2024);
        assert_eq!(path, "puzzle-input/2024/42/input");
    }
}
