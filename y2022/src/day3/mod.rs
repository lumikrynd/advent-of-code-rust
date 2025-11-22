use std::collections::HashSet;

use aoc_helpers::PuzzleSolver;

pub struct Solver {
    rucksacks: Vec<Rucksack>,
}

impl Solver {
    pub fn new(input: &str) -> Box<Self> {
        let rucksacks = input.lines().map(Rucksack::new).collect();
        let solver = Solver { rucksacks };
        Box::new(solver)
    }
}

impl PuzzleSolver for Solver {
    fn solve_part_1(&self) -> Option<String> {
        let value: u32 = self
            .rucksacks
            .iter()
            .map(|r| r.duplicate())
            .map(|c| value(c) as u32)
            .sum();

        //let value = value('A') + value('k');
        Some(format!("{value}"))
    }

    fn solve_part_2(&self) -> Option<String> {
        if !self.rucksacks.len().is_multiple_of(3) {
            panic!("Not possible to group in groups of 3");
        }

        let mut sum = 0;
        let mut i = self.rucksacks.iter();
        while let (Some(a), Some(b), Some(c)) = (i.next(), i.next(), i.next()) {
            let ch = get_intersect_value(a, b, c);
            sum += value(ch) as u32;
        }

        Some(format!("{sum}"))
    }
}

fn get_intersect_value(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> char {
    let a = to_hashset(a.all());
    let b = to_hashset(b.all());
    let c = to_hashset(c.all());

    let result = b.intersection(&c).copied().collect();
    let mut result = a.intersection(&result);

    let (Some(v), None) = (result.next(), result.next()) else {
        panic!("Intersect contains more than one item");
    };

    *v
}

fn value(ch: char) -> u8 {
    match ch {
        'a'..='z' => ch as u8 - b'a' + 1,
        'A'..='Z' => ch as u8 - b'A' + 27,
        _ => panic!("Invalid char {ch}"),
    }
}

fn to_hashset(v: &[char]) -> HashSet<char> {
    v.iter().copied().collect()
}

struct Rucksack(Vec<char>);

impl Rucksack {
    fn new(items: &str) -> Rucksack {
        let items = items.chars().collect();
        Rucksack(items)
    }

    fn duplicate(&self) -> char {
        let s1 = to_hashset(&self.room1());
        let s2 = to_hashset(&self.room2());
        let mut r = s1.intersection(&s2);

        if let (Some(a), None) = (r.next(), r.next()) {
            *a
        } else {
            panic!("Not just a single matching letter")
        }
    }

    fn all(&self) -> &[char] {
        &self.0[..]
    }

    fn room1(&self) -> Vec<char> {
        let l = self.0.len() / 2;
        self.0[..l].to_vec()
    }

    fn room2(&self) -> Vec<char> {
        let l = self.0.len() / 2;
        self.0[l..].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_helpers::PuzzleSolver;

    #[test]
    fn rucksack_works() {
        let items = "aEbd";
        let r = Rucksack::new(items);

        assert_eq!(r.all(), char_arr("aEbd"));
        assert_eq!(r.room1(), char_arr("aE"));
        assert_eq!(r.room2(), char_arr("bd"));
    }

    #[test]
    fn parse_works() {
        let lines = vec!["cialke", "KaZk"];
        let raw = lines.join("\n");

        let result = Solver::new(&raw);

        are_equal(0, &result, &lines);
        are_equal(1, &result, &lines);
    }

    fn are_equal(index: usize, result: &Box<Solver>, lines: &Vec<&str>) {
        assert_eq!(result.rucksacks[index].all(), char_arr(lines[index]));
    }

    #[test]
    fn part_1_calculate_duplicate_value() {
        let lines = vec!["AaZA", "jklFik"];

        let solver = Solver::new(&lines.join("\n"));

        assert_eq!(solver.solve_part_1(), Some(format!("{}", 27 + 11)));
    }

    #[test]
    fn part_2_calculate_badge_value() {
        let lines = vec![
            "AaZA",
            "ZklFik",
            "LOLZ",
            "AaZA",
            "ZklFak",
            "LOLa",
        ];

        let solver = Solver::new(&lines.join("\n"));

        assert_eq!(solver.solve_part_2(), Some(format!("{}", value('Z') + value('a'))));
    }

    fn char_arr(s: &str) -> Vec<char> {
        s.chars().collect()
    }
}
