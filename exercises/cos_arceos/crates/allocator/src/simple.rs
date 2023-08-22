//! Simple memory allocation.
//!
//! TODO: more efficient
use super::{AllocError};
use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    size: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.next = start;
        self.allocations = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if self.start + self.size == start && size > 0 {
            self.size += size;
            AllocResult::Ok(())
        } else {
            AllocResult::Err(AllocError::MemoryOverlap)
            
        }
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        let align = layout.align();

        let aligned_next = (self.next + align - 1) & !(align - 1);

        if aligned_next + size <= self.start + self.size {
            let allocation = self.next;
            self.next = aligned_next + size;
            self.allocations += 1;
            AllocResult::Ok(unsafe { NonZeroUsize::new_unchecked(allocation) })
        } else {
            AllocResult::Err(AllocError::NoMemory)
            
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        self.size - (self.next - self.start)
    }
}