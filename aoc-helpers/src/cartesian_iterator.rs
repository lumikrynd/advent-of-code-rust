pub fn cartesian_set<'l, A, B>(
	a_it: impl Iterator<Item = A> + Clone + 'l,
	b_it: impl Iterator<Item = B> + Clone + 'l,
) -> Box<dyn Iterator<Item = (A, B)> + 'l>
where
	A: Copy + 'l,
	B: Copy + 'l,
{
	let res = b_it.flat_map(move |b| a_it.clone().map(move |a| (a, b)));
	Box::new(res)
}

#[cfg(test)]
mod test {
	use std::cell::RefCell;

	use super::*;

	#[test]
	fn int_iter_test() {
		let a = vec![1, 2, 3];
		let b = vec![4, 5];

		let combined: Vec<_> =
			cartesian_set(a.into_iter(), b.into_iter()).collect();

		let expected = vec![
			(1, 4),
			(2, 4),
			(3, 4),
			(1, 5),
			(2, 5),
			(3, 5),
		];

		assert_eq!(combined, expected);
	}

	#[test]
	fn borrow_iter_test() {
		let a = vec![1, 2, 3];
		let b = vec![4, 5];

		let combined: Vec<_> = cartesian_set(a.iter(), b.iter()).collect();

		let expected = vec![
			(&1, &4),
			(&2, &4),
			(&3, &4),
			(&1, &5),
			(&2, &5),
			(&3, &5),
		];

		assert_eq!(combined, expected);
	}

	/// Testing time isn't used on running through the iterators if the
	/// resulting iterator isn't run through.
	#[test]
	fn is_procedural() {
		let a = vec![1, 2, 3];
		let b = vec![4, 5];

		let a_count = RefCell::new(0);
		let b_count = RefCell::new(0);

		let mut iterator = cartesian_set(
			a.iter().inspect(|_| {
				let mut count = a_count.borrow_mut();
				*count = *count + 1;
			}),
			b.iter().inspect(|_| {
				let mut count = b_count.borrow_mut();
				*count = *count + 1;
			}),
		);

		iterator.next();

		assert_eq!(*a_count.borrow(), 1);
		assert_eq!(*b_count.borrow(), 1);
	}
}
