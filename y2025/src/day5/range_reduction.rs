use super::Id;
use super::Range;

pub fn reduce(ranges: &[Range]) -> Vec<Range> {
	let mut ranges: Vec<_> = ranges.iter().collect();
	ranges.sort_unstable_by_key(|r| r.start());
	ranges.into_iter().fold(Vec::new(), combine)
}

fn combine(mut ranges: Vec<Range>, new: &Range) -> Vec<Range> {
	if let Some(last) = ranges.last_mut()
		&& last.contains(new.start())
	{
		*last = combine_range(last, new);
	} else {
		ranges.push(new.clone());
	}

	ranges
}

fn combine_range(r: &Range, new: &Range) -> Range {
	let new_start = Id::min(*r.start(), *new.start());
	let new_end = Id::max(*r.end(), *new.end());
	new_start..=new_end
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn reduce_non_overlapping() {
		let input = [2..=3, 10..=14];
		assert_eq!(reduce(&input), input);
	}

	#[test]
	fn reduce_single_overlap() {
		let input = [2..=12, 10..=14];
		let expected = [2..=14];
		assert_eq!(reduce(&input), expected);
	}

	#[test]
	fn reduce_single_overlap_edge() {
		let input = [2..=10, 10..=14];
		let expected = [2..=14];
		assert_eq!(reduce(&input), expected);

		let input = [10..=14, 2..=10];
		assert_eq!(reduce(&input), expected);
	}

	#[test]
	fn reduce_single_contains() {
		let input = [2..=12, 3..=5];
		let expected = [2..=12];
		assert_eq!(reduce(&input), expected);

		let input = [3..=5, 2..=12];
		assert_eq!(reduce(&input), expected);
	}

	#[test]
	fn reduce_multi_overlap() {
		let input = [2..=10, 10..=14, 14..=20];
		let expected = [2..=20];
		assert_eq!(reduce(&input), expected);
	}

	#[test]
	fn reduce_new_overlaps_multiple() {
		let input = [2..=11, 13..=20, 10..=14];
		let expected = [2..=20];
		assert_eq!(reduce(&input), expected);
	}
}
