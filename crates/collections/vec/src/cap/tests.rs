// ── Aliases ─────────────────────────────────────────────────────────────────
use super::*;

#[track_caller]
const fn new_cap(val: usize) -> Cap {
	Cap::new(val).expect(
		"\"val\" must be <= Cap::MAX (aka isize::MAX as usize)"
	)
}

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
		fn zero() { assert_eq!(new_cap(0).as_inner(), 0); }

		#[test]
		fn one() { assert_eq!(new_cap(1).as_inner(), 1); }

		#[test]
		fn isize_max() {
			assert_eq!(
				new_cap(isize::MAX as usize).as_inner(),
				isize::MAX as usize
			);
		}
	}
}

// ── `Default` Trait Tests ───────────────────────────────────────────────────
#[test]
fn default() { assert_eq!(Cap::default(), new_cap(0) ); }

// ── `PartialEq` Trait Tests ─────────────────────────────────────────────────
mod partial_eq {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────
	#[test]
	fn true_case() {
		assert_eq!(new_cap(0), new_cap(0));
		assert_eq!(new_cap(1), new_cap(1));
		assert_eq!(new_cap(isize::MAX as usize), new_cap(isize::MAX as usize));
	}

	#[test]
	fn false_case() {
		assert_ne!(new_cap(0), new_cap(1));
		assert_ne!(new_cap(1), new_cap(isize::MAX as usize));
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
			let cap = new_cap(val);
			assert_eq!(cap, cap);
		}
	}

	#[test]
	fn symmetric() {
		assert_eq!(new_cap(1) == new_cap(1), new_cap(1) == new_cap(1));
		assert_eq!(new_cap(0) == new_cap(1), new_cap(1) == new_cap(0));
	}

	#[test]
	fn transitive() {
		let (a, b, c) = (new_cap(1), new_cap(1), new_cap(1));
		assert!(a == b && b == c && a == c);
	}
}

// ── `PartialOrd` Trait Tests ────────────────────────────────────────────────
mod partial_ord {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────
	#[test]
	fn less() {
		assert_eq!(new_cap(0).partial_cmp(&new_cap(1)), Some(Ordering::Less));
		assert!(new_cap(0) < new_cap(1));
		assert!(new_cap(1) <= new_cap(1));
	}

	#[test]
	fn equal() {
		assert_eq!(new_cap(1).partial_cmp(&new_cap(1)), Some(Ordering::Equal));
	}

	#[test]
	fn greater() {
		assert_eq!(
			new_cap(
				isize::MAX as usize
			).partial_cmp(
				&new_cap(1)
			),

			Some(Ordering::Greater)
		);
		assert!(new_cap(isize::MAX as usize) > new_cap(1));
		assert!(new_cap(isize::MAX as usize) >= new_cap(isize::MAX as usize));
	}
}

// ── `Ord` Trait Tests ───────────────────────────────────────────────────────
mod ord {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── Tests ───────────────────────────────────────────────────────────────
	#[test]
	fn less() {
		assert_eq!(new_cap(0).cmp(&new_cap(1)), Ordering::Less);
	}

	#[test]
	fn equal() {
		assert_eq!(new_cap(1).cmp(&new_cap(1)), Ordering::Equal);
	}

	#[test]
	fn greater() {
		assert_eq!(
			new_cap(isize::MAX as usize).cmp(&new_cap(0)),
			Ordering::Greater
		);
	}
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