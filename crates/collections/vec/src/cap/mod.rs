// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    default::Default,
    fmt::Debug,
    mem::transmute,
    option::Option,
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `struct Cap` ────────────────────────────────────────────────────────────
#[derive(Debug)]
pub(super) struct Cap(usize);

// ── `Cap` Implementations ───────────────────────────────────────────────────
impl Cap {
    // ── Constants ───────────────────────────────────────────────────────────
    pub(super) const ZERO: usize = 0;

    pub(super) const MAX: usize = isize::MAX as usize;

    // ── Functions ───────────────────────────────────────────────────────────
    #[inline]
    pub(super) fn new(val: usize) -> Option<Self> {
       if val <= Self::MAX {
           // SAFETY: `val` was just checked and is within the valid range
           Some(unsafe { transmute::<usize, Self>(val) })
       } else {
           None
       }
    }

    #[inline]
    pub(super) unsafe fn new_unchecked(val: usize) -> Self {
        // SAFETY: Caller promised `val` is within [0, isize::max]
        unsafe { transmute::<usize, Self>(val) }
    }

    // ── Methods ─────────────────────────────────────────────────────────────
    #[inline]
    pub(super) fn as_inner(&self) -> usize {
        // SAFETY: `self.0` is guarenteed to be in valid state.
        unsafe { transmute::<&Self, usize>(self) }
    }
}

impl Default for Cap {
    #[inline]
    fn default() -> Cap {
        // SAFETY: 0 is within [0, isize::MAX]
        unsafe { Cap::new_unchecked(0) }
    }
}

impl PartialEq for Cap {
    #[inline]
    fn eq(&self, rhs: &Cap) -> bool { self.as_inner() == rhs.as_inner() }
}
