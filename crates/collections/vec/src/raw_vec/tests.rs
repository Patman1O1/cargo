// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
	alloc::Layout,
	ptr,
	mem
};

use allocator_api2::{
	alloc::Global
};

use super::*;

// ── Functions ───────────────────────────────────────────────────────────────
#[inline]
fn array_layout<T>(n: usize) -> Layout {
	Layout::array::<T>(n).expect("Invalid layout")
}

// ── Non-member Function Tests ───────────────────────────────────────────────
mod min_non_zero_cap {
	use super::*;

	#[test]
	fn elem_size_0() { assert_eq!(min_non_zero_cap(0), 4); }

	#[test]
	fn elem_size_1() { assert_eq!(min_non_zero_cap(1), 8); }

	#[test]
	fn elem_size_2() { assert_eq!(min_non_zero_cap(2), 4); }

	#[test]
	fn elem_size_1024() { assert_eq!(min_non_zero_cap(1024), 4); }

	#[test]
	fn elem_size_1025() { assert_eq!(min_non_zero_cap(1025), 1); }

	#[test]
	fn elem_size_usize_max() { assert_eq!(min_non_zero_cap(usize::MAX), 1); }
}

// ── `RawVec` Function Tests ─────────────────────────────────────────────────
mod new {
	use super::*;

	#[test]
	fn test() {
		let vec = RawVec::new(array_layout::<i32>(4));

		assert_ne!(vec.ptr::<i32>(), ptr::null());
		assert_eq!(vec.cap, 0);
	}
}
