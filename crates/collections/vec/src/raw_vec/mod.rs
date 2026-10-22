// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        self,
        Layout
    },
    cmp,
    hint,
    num::NonZero,
    ptr::{
        self,
        NonNull
    }
};
use std::alloc::handle_alloc_error;
use allocator_api2::{
    alloc::{
        Allocator,
        Global
    },
    collections::{
        TryReserveError,
        TryReserveErrorKind,
        TryReserveErrorKind::AllocError,
        TryReserveErrorKind::CapacityOverflow
    }
};

use crate::alloc_init::AllocInit;

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `struct RawVec` Definition ──────────────────────────────────────────────
pub(crate) struct RawVec<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: usize,
    alloc: A
}

// ── `RawVec<Global>` Implementation ─────────────────────────────────────────
impl RawVec<Global> {
    // ── Functions ───────────────────────────────────────────────────────────
    #[inline]
    pub(crate) const fn new(elem_layout: Layout) -> Self {
        Self {
            ptr: dangling(elem_layout),
            cap: 0,
            alloc: Global
        }
    }

    #[inline]
    pub(crate) fn try_with_capacity(
        capacity: usize,
        elem_layout: Layout
    ) -> Result<Self, TryReserveError> {
        Self::try_allocate_in(
            capacity,
            AllocInit::Uninitialized,
            Global,
            elem_layout
        )
    }


}

// ── `RawVec<A>` Implementation ──────────────────────────────────────────────
impl<A: Allocator> RawVec<A> {
    // ── Constants ───────────────────────────────────────────────────────────
    pub(crate) const MAX_CAP: usize = isize::MAX as usize;

    // ── Functions ───────────────────────────────────────────────────────────
    #[inline]
    pub(crate) const fn new_in(
        alloc: A,
        elem_layout: Layout
    ) -> Self {
        Self {
            ptr: dangling(elem_layout),
            cap: 0,
            alloc
        }
    }

    fn try_allocate_in(
        capacity: usize,
        init: AllocInit,
        alloc: A,
        elem_layout: Layout
    ) -> Result<Self, TryReserveError> {
        let layout = array_layout(capacity, elem_layout)?;

        if layout.size() == 0 {
            return Ok(Self::new_in(alloc, elem_layout));
        }

        let result = match init {
            AllocInit::Uninitialized => alloc.allocate(layout),
            AllocInit::Zeroed => alloc.allocate_zeroed(layout),
        };

        match result {
            Ok(ptr) => Ok(Self { ptr: ptr.cast(), cap: capacity, alloc }),
            Err(_) => {
                hint::cold_path();
                Err(AllocError { layout, non_exhaustive: () }.into())
            }
        }
    }

    // ── Methods ─────────────────────────────────────────────────────────────
    #[inline]
    pub(crate) const fn allocator(&self) -> &A { &self.alloc }

    #[inline]
    pub(crate) const fn capacity(&self, elem_size: usize) -> usize {
        match elem_size {
            0 => usize::MAX,
            _ => self.cap
        }
    }

    #[inline]
    pub(crate) const fn layout(
        &self,
        elem_layout: Layout
    ) -> Option<Layout> {
        if self.cap == 0 || elem_layout.size() == 0 {
            hint::cold_path();
            return None;
        }

        Some(
            // SAFETY: `elem_layout`
            unsafe {
                Layout::from_size_align_unchecked(
                    self.cap.unchecked_mul(elem_layout.size()),
                    elem_layout.align()
                )
            }
        )
    }

    #[inline]
    pub(crate) const fn non_null<T>(&self) -> NonNull<T> { self.ptr.cast() }

    #[inline]
    pub(crate) const fn ptr<T>(&self) -> *const T { self.ptr.as_ptr().cast() }

    #[inline]
    pub(crate) const fn ptr_mut<T>(&self) -> *mut T {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    const fn needs_to_grow(
        &self,
        len: usize,
        additional: usize,
        elem_layout: Layout
    ) -> bool {
        additional > self.capacity(elem_layout.size()).wrapping_sub(len)
    }

    #[cold]
    unsafe fn grow(
        &self,
        new_cap: usize,
        elem_layout: Layout
    ) -> Result<NonNull<u8>, TryReserveError> {
        let new_layout = array_layout(new_cap, elem_layout)?;

        let result = match self.layout(elem_layout) {
            Some(curr_layout) => unsafe {
                hint::assert_unchecked(
                    curr_layout.align() == new_layout.align()
                );
                self.alloc.grow(self.ptr, curr_layout, new_layout)
            },
            None => {
                self.alloc.allocate(new_layout)
            }
        };
        
        match result {
            Ok(ptr) => Ok(ptr.cast()),
            Err(_) => {
                hint::cold_path();
                Err(AllocError { layout: new_layout, non_exhaustive: () }.into())
            }
        }
    }

    unsafe fn grow_exact(
        &mut self,
        len: usize,
        additional: usize,
        elem_layout: Layout
    ) -> Result<(), TryReserveError> {
        if elem_layout.size() == 0 {
            hint::cold_path();
            return Err(CapacityOverflow.into());
        }

        self.cap = len.checked_add(additional).ok_or(CapacityOverflow)?;

        // SAFETY: `self.cap >= len + additional`. All other preconditions
        // passed to the caller.
        self.ptr = unsafe { self.grow(self.cap, elem_layout) }?;

        Ok(())
    }

    unsafe fn grow_amortized(
        &mut self,
        len: usize,
        additional: usize,
        elem_layout: Layout
    ) -> Result<(), TryReserveError> {
        if elem_layout.size() == 0 {
            hint::cold_path();
            return Err(CapacityOverflow.into());
        }

        let required_cap = len.checked_add(
            additional
        ).ok_or(
            CapacityOverflow
        )?;

        self.cap = cmp::max(
            min_non_zero_cap(elem_layout.size()),
            cmp::max(self.cap << 1, required_cap)
        );
        self.ptr = unsafe { self.grow(self.cap, elem_layout)? };

        Ok(())
    }

    /// # Safety
    /// - `new_cap <= self.capacity()`
    unsafe fn shrink_unchecked(
        &mut self,
        new_cap: usize,
        elem_layout: Layout
    ) -> Result<(), TryReserveError> {
        let curr_layout = self.layout(elem_layout).ok_or(CapacityOverflow)?;
        
        if new_cap == 0 {
            unsafe { self.alloc.deallocate(self.ptr, curr_layout) };

            // SAFETY: `elem_layout.align()` always returns a nonzero value.
            self.ptr = unsafe {
                dangling(elem_layout)
            };

            self.cap = 0;
        } else {
            let new_size = unsafe {
                new_cap.unchecked_mul(elem_layout.size())
            };
            let new_layout = array_layout(new_size, elem_layout)?;
            let curr_layout = self.layout(
                elem_layout
            ).ok_or(
                CapacityOverflow
            )?;

            self.cap = new_cap;
            self.ptr = unsafe {
                self.alloc.shrink(
                    self.ptr,
                    curr_layout,
                    new_layout
                ).map_err(
                    |_| AllocError {
                        layout: new_layout,
                        non_exhaustive: ()
                    }
                )?.cast()
            };
        }
        
        Ok(())
    }

    unsafe fn deallocate(&mut self, elem_layout: Layout) {
        if let Some(layout) = self.layout(elem_layout) {
            unsafe {
                self.alloc.deallocate(self.ptr, layout);
            }
        }
    }
}

// ── Non-member Functions ────────────────────────────────────────────────────
#[inline]
const fn min_non_zero_cap(elem_size: usize) -> usize {
    if elem_size == 1 {
        8
    } else if elem_size <= 1024 {
        4
    } else {
        1
    }
}

#[inline]
const fn array_layout(
    cap: usize,
    elem_layout: Layout
) -> Result<Layout, TryReserveErrorKind> {
    let Some(size) = cap.checked_mul(elem_layout.size()) else {
        return Err(CapacityOverflow);
    };

    match Layout::from_size_align(size, elem_layout.align()) {
        Ok(layout) => Ok(layout),
        Err(_) => Err(CapacityOverflow)
    }
}

/// # Safety
/// - `align` must be a nonzero power of two.
#[inline]
const fn dangling(elem_layout: Layout) -> NonNull<u8> {
    // SAFETY: `elem_layout.align()` is a power of two.
    unsafe {
        NonNull::new_unchecked(
            ptr::without_provenance_mut(elem_layout.align())
        )
    }
}


#[cfg(not(no_global_oom_handling))]
#[inline(never)]
const fn handle_cap_overflow_error() -> ! { panic!("capacity overflow") }

#[cfg(not(no_global_oom_handling))]
#[cold]
fn handle_error(err: TryReserveError) -> ! {
    match err.kind() {
        CapacityOverflow => handle_cap_overflow_error(),
        AllocError { layout, .. } => handle_alloc_error(layout)
    }
}