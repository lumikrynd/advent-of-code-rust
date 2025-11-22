use super::*;

pub fn parse(input: &str) -> Vec<Strategy> {
    input.lines()
        .map(parse_single)
        .collect()
}

fn parse_single(input: &str) -> Strategy {
    let input: Vec<char> = input.chars().collect();
    let opponent = parse_move(input[0]);
    let response = parse_response(input[2]);
    Strategy::new(opponent, response)
}

fn parse_response(char: char) -> Response {
    match char {
        'X' => Response::X,
        'Y' => Response::Y,
        'Z' => Response::Z,
        c => panic!("unknown input '{}'", c),
    }
}

fn parse_move(char: char) -> Move {
    match char {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        c => panic!("unknown input '{}'", c),
    }
}

#[cfg(test)]
mod test {
    use crate::day2::parsing::{Move, Response, Strategy, parse};

    #[test]
    fn name() {
        let input = "A Y\nB X\nC Z";
        let result = parse(input);

        let expected = vec![
            Strategy::new(Move::Rock, Response::Y),
            Strategy::new(Move::Paper, Response::X),
            Strategy::new(Move::Scissors, Response::Z),
        ];

        assert_eq!(result, expected);
    }
}
