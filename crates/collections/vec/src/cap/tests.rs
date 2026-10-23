// ── Aliases ─────────────────────────────────────────────────────────────────
use super::*;

// ── `Cap` Function Tests ────────────────────────────────────────────────────
mod functions {
	// ── Aliases ─────────────────────────────────────────────────────────────
	use super::*;

	// ── `Cap::new()` Tests ──────────────────────────────────────────────────
	mod new {
		use super::*;

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

	mod new_unchecked {
		use super::*;

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

// ── `PartialEq` Trait Tests ─────────────────────────────────────────────────

// ── `Eq` Trait Tests ────────────────────────────────────────────────────────

// ── `PartialOrd` Trait Tests ────────────────────────────────────────────────

// ── `Ord` Trait Tests ───────────────────────────────────────────────────────

// ── `Hash` Trait Tests ──────────────────────────────────────────────────────

// ── `Display` Trait Tests ───────────────────────────────────────────────────

// ── `Debug` Trait Tests ─────────────────────────────────────────────────────
