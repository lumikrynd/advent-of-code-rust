use crate::day4::ElfPair;

use super::Range;

pub fn parse(input: &str) -> Vec<ElfPair> {
    input.lines().map(parse_single).collect()
}

fn parse_single(input: &str) -> ElfPair {
    let splits: Vec<_> = input.split(',').collect();
    let [a, b] = splits[..] else {
        panic!("Not a pair")
    };

    let a = parse_range(a);
    let b = parse_range(b);

    (a, b)
}

fn parse_range(input: &str) -> Range {
    let splits: Vec<_> = input.split('-').collect();
    let [a, b] = splits[..] else {
        panic!("Not a pair")
    };
    let a = a.parse().unwrap();
    let b = b.parse().unwrap();
    Range::new(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "2-4,6-8\n2-3,4-5";
        let result = parse(input);

        let expected = vec![
            (Range::new(2, 4), Range::new(6, 8)),
            (Range::new(2, 3), Range::new(4, 5)),
        ];

        assert_eq!(result, expected);
    }
}
