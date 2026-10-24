// ── Aliases ─────────────────────────────────────────────────────────────────
use super::*;

// ── `Cap` Function Tests ────────────────────────────────────────────────────
mod functions {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── `Cap::new()` Tests ──────────────────────────────────────────────────
	mod new {
		// ── Aliases ─────────────────────────────────────────────────────────
		use super::*;

		// ── Tests ───────────────────────────────────────────────────────────
		#[test]
		fn zero() { assert_ne!(None, Cap::new(0)); }

		#[test]
		fn one() { assert_ne!(None, Cap::new(1)); }

		#[test]
		fn isize_max() {
			assert_ne!(None, Cap::new(isize::MAX as usize));
		}

		#[test]
		fn isize_max_plus_one() {
			assert_eq!(None, Cap::new(isize::MAX as usize + 1));
		}

		#[test]
		fn usize_max() { assert_eq!(None, Cap::new(usize::MAX)); }
	}

	// ── `Cap::new_unchecked()` Tests ────────────────────────────────────────
	mod new_unchecked {
		// ── Aliases ─────────────────────────────────────────────────────────
		use super::*;

		// ── Tests ───────────────────────────────────────────────────────────
		#[test]
		fn zero() {
			// SAFETY: `0` <= `Cap::MAX` (aka `isize::max as usize`)
			unsafe {
				assert_eq!(
					Cap::new_unchecked(0).as_inner(),
					0
				);
			}
		}

		#[test]
		fn one() {
			// SAFETY: `1` <= `Cap::MAX` (aka `isize::max as usize`)
			unsafe {
				assert_eq!(
					Cap::new(1).unwrap_unchecked().as_inner(),
					1
				)
			}
		}

		#[test]
		fn isize_max() {
			// SAFETY: `isize::MAX as usize` <= `Cap::MAX`
			// (aka `isize::max as usize`).
			unsafe {
				assert_eq!(
					Cap::new_unchecked(isize::MAX as usize).as_inner(),
					isize::MAX as usize
				)
			}
		}
	}
}

// ── `Cap` Methods Tests ─────────────────────────────────────────────────────
mod methods {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── `Cap::as_inner()` ───────────────────────────────────────────────────
	mod as_inner {
		// ── Aliases ─────────────────────────────────────────────────────────
		use super::*;

		// ── Tests ───────────────────────────────────────────────────────────
		#[test]
		fn zero() {
			// SAFETY: `0` <= `Cap::MAX` (aka `isize::max as usize`)
			unsafe {
				assert_eq!(
					Cap::new_unchecked(0).as_inner(),
					0
				);
			}
		}

		#[test]
		fn one() {
			// SAFETY: `1` <= `Cap::MAX` (aka `isize::max as usize`)
			unsafe {
				assert_eq!(
					Cap::new(1).unwrap_unchecked().as_inner(),
					1
				)
			}
		}

		#[test]
		fn isize_max() {
			// SAFETY: `isize::MAX as usize` <= `Cap::MAX`
			// (aka `isize::max as usize`).
			unsafe {
				assert_eq!(
					Cap::new_unchecked(isize::MAX as usize).as_inner(),
					isize::MAX as usize
				)
			}
		}
	}
}

// ── `Default` Trait Tests ───────────────────────────────────────────────────
#[test]
fn default() {
	// SAFETY: `0` <= `Cap::MAX` (aka `isize::MAX as usize`).
	assert_eq!(Cap::default(), unsafe { Cap::new_unchecked(0)} );
}

// ── `PartialEq` Trait Tests ─────────────────────────────────────────────────
mod partial_eq {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────
	#[test]
	fn true_case() {
		unsafe {
			assert_eq!(Cap::new_unchecked(0), Cap::new_unchecked(0));
			assert_eq!(Cap::new_unchecked(1), Cap::new_unchecked(1));
			assert_eq!(
				Cap::new_unchecked(isize::MAX as usize),
				Cap::new_unchecked(isize::MAX as usize)
			);
		}
	}

	#[test]
	fn false_case() {
		unsafe {
			assert_ne!(Cap::new_unchecked(0), Cap::new_unchecked(1));
			assert_ne!(
				Cap::new_unchecked(1),
				Cap::new_unchecked(isize::MAX as usize)
			);
		}
	}
}

// ── `Eq` Trait Tests ────────────────────────────────────────────────────────
mod eq {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────
	#[test]
	fn reflexive() {
		for val in [0, 1, isize::MAX as usize] {
			let cap = unsafe { Cap::new_unchecked(val) };
			assert_eq!(cap, cap);
		}
	}

	#[test]
	fn symmetric() {
		unsafe {
			assert_eq!(
				Cap::new_unchecked(1) == Cap::new_unchecked(1),
				Cap::new_unchecked(1) == Cap::new_unchecked(1)
			);

			assert_eq!(
				Cap::new_unchecked(0) == Cap::new_unchecked(1),
				Cap::new_unchecked(1) == Cap::new_unchecked(0)
			);
		}
	}

	#[test]
	fn transitive() {
		let (a, b, c) = unsafe {
			(
				Cap::new_unchecked(1),
				Cap::new_unchecked(1),
				Cap::new_unchecked(1)
			)
		};

		assert!(a == b && b == c && a == c);
	}
}

// ── `PartialOrd` Trait Tests ────────────────────────────────────────────────
mod partial_ord {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────

}

// ── `Ord` Trait Tests ───────────────────────────────────────────────────────
mod ord {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────

}

// ── `Hash` Trait Tests ──────────────────────────────────────────────────────
mod hash {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────

}

// ── `Display` Trait Tests ───────────────────────────────────────────────────
mod display {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────

}

// ── `Debug` Trait Tests ─────────────────────────────────────────────────────
mod debug {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────

}