/// Creates a struct containing a single type, which implements Deref and DerefMut.
/// Useful for having strict typing.
/// Argument 1 is the struct name, Argument 2 the type it wraps.
/// Any additional arguments will be passed to #[derive].
/// PartialEq and Clone are already passed to derive and will cause an error.
#[macro_export]
macro_rules! wrapper {
	($i:ident, $t:ty $(, $item:ident) *) => {
		#[derive(PartialEq, Clone $(, $item)*)]
		struct $i($t);
		$crate::wrapper_inner!($i, $t);
	};
}

#[macro_export]
macro_rules! pub_wrapper {
	($i:ident, $t:ty $(, $item:ident) *) => {
		#[derive(PartialEq, Clone $(, $item)*)]
		pub struct $i(pub $t);
		$crate::wrapper_inner!($i, $t);
	};
}

#[macro_export]
macro_rules! wrapper_inner {
	($i:ident, $t:ty $(, $item:ident) *) => {
		impl std::ops::Deref for $i {
			type Target = $t;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl std::ops::DerefMut for $i {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};
}

#[cfg(test)]
mod test {
	wrapper!(WrapStr, &'static str);
	wrapper!(WrapString, String);

	#[test]
	fn deref() {
		let inner = "hallo";
		let wrapper = WrapStr(inner);

		assert_eq!(inner, *wrapper);
	}

	#[test]
	fn deref_mut() {
		let inner = "hallo".to_string();
		let mut wrapper = WrapString(inner);

		{
			let k = &mut wrapper;
			k.0.push_str("\nWithout mut deref");
			k.push_str("\nWith mut deref");
		}

		assert_eq!("hallo\nWithout mut deref\nWith mut deref", wrapper.0)
	}

	// TODO: Find a way to implement display and debug if inner
	// type implements them
	// ... or just use the derive thing which results in a slightly different
	// result... Might play around with it later.
	/*
	#[test]
	fn display() {
		let inner = "hallo";
		let wrapper = WrapStr(inner);

		assert_eq!(format!("{inner}"), format!("{wrapper}"));
	}

	#[test]
	fn debug() {
		let inner = "hallo";
		let wrapper = WrapStr(inner);

		assert_eq!(format!("{inner:?}"), format!("{wrapper:?}"));
	}

	#[test]
	fn pretty_print() {
		let inner = "hallo";
		let wrapper = WrapStr(inner);

		assert_eq!(format!("{inner:#?}"), format!("{wrapper:#?}"));
	}
	*/
}
