use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T>
where
	T: Clone + Copy + Debug + PartialEq + Eq + Hash,
{
	x: T,
	y: T,
}

impl<T> Point2D<T>
where
	T: Clone + Copy + Debug + PartialEq + Eq + Hash + Add + Mul + Sub + Div,
{
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}

	pub fn x(&self) -> T {
		self.x
	}

	pub fn y(&self) -> T {
		self.y
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point3D {
	x: i32,
	y: i32,
	z: i32,
}

impl Point3D {
	pub fn new(x: i32, y: i32, z: i32) -> Self {
		Self { x, y, z }
	}

	pub fn x(&self) -> i32 {
		self.x
	}

	pub fn y(&self) -> i32 {
		self.y
	}

	pub fn z(&self) -> i32 {
		self.z
	}

	pub fn distance(&self, other: &Self) -> f64 {
		let sum = dist_pow(self.x, other.x)
			+ dist_pow(self.y, other.y)
			+ dist_pow(self.z, other.z);

		f64::sqrt(sum)
	}
}

fn dist_pow(a: i32, b: i32) -> f64 {
	let v = a - b;
	let v : f64 = v.into();
	v.powi(2)
}

#[cfg(test)]
mod test {
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn can_be_used_as_key_2d() {
		let mut map = HashSet::new();
		assert!(map.insert(new_2d(1, 2)));
		assert!(!map.insert(new_2d(1, 2)));
		assert!(map.insert(new_2d(2, 2)));
		assert!(map.insert(new_2d(1, 3)));
		assert!(!map.insert(new_2d(2, 2)));
	}

	fn new_2d(x: usize, y: usize) -> Point2D<usize> {
		Point2D::new(x, y)
	}

	#[test]
	fn can_be_used_as_key_3d() {
		let mut map = HashSet::new();
		assert!(map.insert(new_3d(1, 2, 0)));
		assert!(!map.insert(new_3d(1, 2, 0)));
		assert!(map.insert(new_3d(2, 2, 0)));
		assert!(map.insert(new_3d(1, 3, 0)));
		assert!(!map.insert(new_3d(2, 2, 0)));
		assert!(map.insert(new_3d(1, 2, 1)));
	}

	#[test]
	fn distance() {
		let a = new_3d(1, 1, 1);
		let b = new_3d(4, 5, 1);

		assert_eq!(a.distance(&b), 5.0);
		assert_eq!(b.distance(&a), 5.0);
	}

	fn new_3d(x: i32, y: i32, z: i32) -> Point3D {
		Point3D::new(x, y, z)
	}
}
