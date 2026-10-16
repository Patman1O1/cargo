// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Layout,
        LayoutError
    },
    hint::cold_path,
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
        TryReserveErrorKind::AllocError,
        TryReserveErrorKind::CapacityOverflow
    }
};

use crate::alloc_init::AllocInit;

// ── Modules ─────────────────────────────────────────────────────────────────


// ── `struct RawVec` Definition ──────────────────────────────────────────────
pub(crate) struct RawVec<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: usize,
    alloc: A
}

// ── `RawVec` Implementation ─────────────────────────────────────────────────
impl<A: Allocator> RawVec<A> {
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
        elem_layout: Layout
    ) -> Result<Self, TryReserveError> {
        if capacity > isize::MAX as usize {
            cold_path();
            return Err(CapacityOverflow.into())
        }

        let layout = match Layout::array::<u8>(
            capacity * elem_layout.size()
        ) {
            Ok(layout) => layout,
            Err(_) => {
                cold_path();
                return Err(CapacityOverflow.into());
            }
        };

        if layout.size() == 0 {
            cold_path();
            return Ok(Self::new_in(alloc, elem_layout));
        }

        let result = match init {
            AllocInit::Uninitialized => alloc.allocate(layout),
            AllocInit::Zeroed => alloc.allocate_zeroed(layout)
        };

        Ok(Self {
            ptr: match result {
                Ok(ptr) => ptr.cast(),
                Err(_) => return Err(
                    AllocError { layout, non_exhaustive: () }.into()
                )
            },
            cap: capacity,
            alloc
        })
    }

    // ── Methods ─────────────────────────────────────────────────────────────
    #[inline]
    pub(crate) const fn allocator(&self) -> &A { &self.alloc }

    #[inline]
    pub(crate) const fn non_null<T>(&self) -> NonNull<T> { self.ptr.cast() }

    #[inline]
    pub(crate) const fn ptr<T>(&self) -> *mut T { self.ptr.as_ptr() as *mut T }

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
    ) -> Result<Layout, LayoutError> {
        Layout::array::<u8>(self.cap * elem_layout.size())
    }
}
