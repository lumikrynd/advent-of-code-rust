mod parsing;

pub fn solve(input: &str) -> String {
    let strategies = parsing::parse(input);
    let part_1 = solve_part_1(&strategies);
    let part_2 = solve_part_2(&strategies);

    format!("part1: {}\npart2: {}", part_1, part_2)
}

fn solve_part_1(strategies: &[Strategy]) -> String {
    let result: u32 = strategies
        .iter()
        .map(|s| (s.response.to_move(), &s.opponent))
        .map(|s| s.0.point() + result_against(&s.0, s.1).point())
        .sum();

    result.to_string()
}

fn solve_part_2(strategies: &[Strategy]) -> String {
    let result: u32 = strategies
        .iter()
        .map(|s| (s.response.to_result(), &s.opponent))
        .map(|s| get_move_from_result(&s.0, s.1).point() + s.0.point())
        .sum();

    result.to_string()
}

impl Response {
    fn to_move(&self) -> Move {
        match self {
            Response::X => Move::Rock,
            Response::Y => Move::Paper,
            Response::Z => Move::Scissors,
        }
    }

    fn to_result(&self) -> Result {
        match self {
            Response::X => Result::Loose,
            Response::Y => Result::Draw,
            Response::Z => Result::Win,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug, Clone)]
enum Result {
    Win,
    Loose,
    Draw,
}

impl Result {
    fn point(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Loose => 0,
            Result::Draw => 3,
        }
    }
}

impl Move {
    fn point(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

const GAMES: [(Move, Move, Result); 9] = [
    (Move::Rock, Move::Rock, Result::Draw),
    (Move::Rock, Move::Paper, Result::Loose),
    (Move::Rock, Move::Scissors, Result::Win),
    (Move::Paper, Move::Rock, Result::Win),
    (Move::Paper, Move::Paper, Result::Draw),
    (Move::Paper, Move::Scissors, Result::Loose),
    (Move::Scissors, Move::Rock, Result::Loose),
    (Move::Scissors, Move::Paper, Result::Win),
    (Move::Scissors, Move::Scissors, Result::Draw),
];

fn get_move_from_result(result: &Result, opponent: &Move) -> Move {
    GAMES
        .iter()
        .filter(|g| &g.1 == opponent && result == &g.2)
        .map(|g| &g.0)
        .next()
        .expect("")
        .clone()
}

fn result_against(me: &Move, opponent: &Move) -> Result {
    GAMES
        .iter()
        .filter(|g| &g.0 == me && &g.1 == opponent)
        .map(|g| &g.2)
        .next()
        .expect("")
        .clone()
}

#[derive(PartialEq, Debug)]
enum Response {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Debug)]
pub struct Strategy {
    opponent: Move,
    response: Response,
}

impl Strategy {
    fn new(opponent: Move, response: Response) -> Self {
        Strategy { opponent, response }
    }
}
