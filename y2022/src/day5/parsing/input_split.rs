pub fn split_into_sections(input: &str) -> Vec<Vec<&str>> {
	let not_empty_line = |s: &&str| !s.trim().is_empty();
	let transform = Split::insert_if_else_next_pred(not_empty_line);
	input.lines().fold(Split::<&str>::new(), transform).unpack()
}

struct Split<T> {
	groups: Vec<Vec<T>>,
}

impl<T> Split<T> {
	fn unpack(self) -> Vec<Vec<T>> {
		self.groups
	}

	fn new() -> Self {
		let groups = vec![vec![]];
		Self { groups }
	}

	fn insert(mut self, value: T) -> Self {
		self.groups.last_mut().unwrap().push(value);
		self
	}

	fn next(mut self) -> Self {
		self.groups.push(Vec::new());
		self
	}

	fn insert_if_else_next<F>(self, value: T, pred: &F) -> Self
	where
		F: Fn(&T) -> bool,
	{
		if (*pred)(&value) {
			self.insert(value)
		} else {
			self.next()
		}
	}

	/// Takes in a predicate for which lines should split the input,
	/// and returns a predicate which can be used with `Fold` to achieve that
	/// result
	/// On true the value is added to the current collection, on false the next
	/// collection is created.
	fn insert_if_else_next_pred<F>(pred: F) -> impl FnMut(Self, T) -> Self
	where
		F: Fn(&T) -> bool,
	{
		move |a, b| a.insert_if_else_next(b, &pred)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn split_test_on_emtpy_lines() {
		let input = "a\nb\n\nc\nd\ne\n\nf";

		let not_empty = |s: &&str| !s.trim().is_empty();
		let transform = Split::insert_if_else_next_pred(not_empty);

		let result = input.lines().fold(Split::<&str>::new(), transform).unpack();
		let mut r = result.iter();

		let (Some(a), Some(b), Some(c), None) = (r.next(), r.next(), r.next(), r.next()) else {
			panic!("Whoops");
		};
		assert_eq!(*a, vec!["a", "b"]);
		assert_eq!(*b, vec!["c", "d", "e"]);
		assert_eq!(*c, vec!["f"]);
	}
}
