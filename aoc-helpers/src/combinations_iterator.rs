/// Compute all combinations of the values given in the iterator.
pub fn combinations_set<'l, A>(
	a_it: impl Iterator<Item = A> + Clone + 'l,
) -> Box<dyn Iterator<Item = (A, A)> + 'l>
where
	A: Copy + 'l,
{
	let b_it = a_it.clone();
	let res = a_it.enumerate().flat_map(move |(count, a)| {
		b_it.clone().skip(count + 1).map(move |b| (a, b))
	});
	Box::new(res)
}

#[cfg(test)]
mod test {
	use std::cell::RefCell;

	use super::*;

	#[test]
	fn int_iter_test() {
		let a = [1, 2, 3];

		let combined: Vec<_> = combinations_set(a.into_iter()).collect();

		let expected = [(1, 2), (1, 3), (2, 3)];

		assert_eq!(combined, expected);
	}

	#[test]
	fn borrow_iter_test() {
		let a = [1, 2, 3];

		let combined: Vec<_> = combinations_set(a.iter()).collect();

		let expected = [(&1, &2), (&1, &3), (&2, &3)];

		assert_eq!(combined, expected);
	}

	/// Testing time isn't used on running through the iterators if the
	/// resulting iterator isn't run through.
	#[test]
	fn is_procedural() {
		let a = [1, 2, 3];

		let a_count = RefCell::new(0);

		let mut iterator = combinations_set(a.iter().inspect(|_| {
			let mut count = a_count.borrow_mut();
			*count += 1;
		}));

		iterator.next();
		assert_eq!(*a_count.borrow(), 3); //1 from each + 1 for skip
	}
}
