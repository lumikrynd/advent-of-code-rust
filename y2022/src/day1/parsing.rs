use super::Elf;

pub fn parse(raw: &str) -> Vec<Elf> {
    let mut elfs = Vec::new();
    let mut lines = raw.lines();

    while let Some(e) = parse_elf(&mut lines) {
        elfs.push(e);
    }

    elfs
}

fn parse_elf(lines: &mut std::str::Lines<'_>) -> Option<Elf> {
    let mut foods = Vec::new();
    while let Some(food) = parse_food(lines) {
        foods.push(food);
    }

    if !foods.is_empty() {
        Some(Elf::new(foods))
    } else {
        None
    }
}

fn parse_food(lines: &mut std::str::Lines<'_>) -> Option<u32> {
    let line = lines.next()?;

    if let Result::Ok(i) = line.parse::<u32>() {
        Some(i)
    } else if line.is_empty() {
        None
    } else {
        panic!("INVALID INPUT")
    }
}

impl Elf {
    fn new(food: Vec<u32>) -> Self {
        Elf { food }
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.food == other.food
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_in_empty_out() {
        let input = "";
        let result = parse(input);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn single_food_item() {
        let input = "1000";
        let result = parse(input);
        let expected = vec![Elf::new(vec![1000])];
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_elfs_item() {
        let input = "1000\n\n4000\n\n2000";
        let result = parse(input);

        let expected = vec![
            Elf::new(vec![1000]),
            Elf::new(vec![4000]),
            Elf::new(vec![2000]),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn invalid_input_space_before_number() {
        // N
        let input = " 1000";
        parse(input);
    }

    #[test]
    fn multiple_elfs_with_multiple_item() {
        let input = "1000\n\n4000\n3000\n\n2000";
        let result = parse(input);

        let expected = vec![
            Elf::new(vec![1000]),
            Elf::new(vec![4000, 3000]),
            Elf::new(vec![2000]),
        ];

        assert_eq!(result, expected);
    }
}
