mod parsing;

pub fn solve(input: &str) -> String {
    let elfs = parsing::parse(input);
    let part_1 = solve_part_1(elfs);

    format!("part1: {}", part_1)
}

fn solve_part_1(elfs: Vec<Elf>) -> String {
    let result = elfs.iter()
        .map(|e| e.sum())
        .max()
        .unwrap();

    result.to_string()
}

#[derive(Debug)]
struct Elf {
    food: Vec<u32>,
}

impl Elf {
    fn sum(&self) -> u32 {
        self.food.iter().sum()
    }
}
