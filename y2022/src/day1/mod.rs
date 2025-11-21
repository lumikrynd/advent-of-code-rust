use std::collections::BinaryHeap;

mod parsing;

pub fn solve(input: &str) -> String {
    let elfs = parsing::parse(input);
    let part_1 = solve_part_1(&elfs);
    let part_2 = solve_part_2(&elfs);

    format!("part1: {}\npart2: {}", part_1, part_2)
}

fn solve_part_1(elfs: &Vec<Elf>) -> String {
    let result = elfs.iter()
        .map(|e| e.sum())
        .max()
        .unwrap();

    result.to_string()
}

fn solve_part_2(elfs: &Vec<Elf>) -> String {
    let values = elfs.iter().map(|e| e.sum());
    let sorted = BinaryHeap::from_iter(values);

    let result : u32 = sorted.iter().take(3).sum();
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
