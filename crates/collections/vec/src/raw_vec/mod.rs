// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Layout
    },
    hint,
    num::NonZero,
    ptr::NonNull
};

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
mod reference; // temporary

#[cfg(test)]
mod tests;

// ── `struct RawVec` Definition ──────────────────────────────────────────────
pub(crate) struct RawVec<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: usize,
    alloc: A
}

// ── `RawVec` Implementation ─────────────────────────────────────────────────
impl<A: Allocator> RawVec<A> {
    // ── Constants ───────────────────────────────────────────────────────────
    pub(crate) const MAX_CAP: usize = isize::MAX as usize;

    // ── Functions ───────────────────────────────────────────────────────────
    #[inline]
    pub(crate) const fn new_in(
        alloc: A,
        elem_layout: Layout,
    ) -> Self {
        Self {
            ptr: NonNull::without_provenance(
                // SAFETY: `Layout::align()` is always non-zero.
                unsafe { NonZero::<usize>::new_unchecked(elem_layout.align()) }
            ),
            cap: 0,
            alloc
        }
    }

    pub(crate) fn try_allocate_in(
        capacity: usize,
        init: AllocInit,
        alloc: A,
        elem_layout: Layout,
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

        Ok(())
    }

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
    pub(crate) const fn raw_non_null(&self) -> NonNull<u8> { self.ptr }

    #[inline]
    pub(crate) const fn raw_ptr(&self) -> *mut u8 { self.ptr.as_ptr() }

    #[inline]
    pub(crate) const fn non_null<T>(&self) -> NonNull<T> { self.ptr.cast() }

    #[inline]
    pub(crate) const fn ptr<T>(&self) -> *mut T { self.ptr.as_ptr().cast() }
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
