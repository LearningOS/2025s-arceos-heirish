#![no_std]

use core::ptr::NonNull;

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start_vaddr: usize,
    curr_bytes_pos: usize,
    curr_page_pos: usize,
    page_pos_end: usize,
    end_vaddr: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start_vaddr: 0,
            curr_bytes_pos: 0,
            curr_page_pos: 0,
            page_pos_end: 0,
            end_vaddr: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start_vaddr = start;
        self.curr_bytes_pos = self.start_vaddr;
        self.end_vaddr = start + size;
        self.page_pos_end = align_down(self.end_vaddr, PAGE_SIZE);
        self.curr_page_pos = self.end_vaddr
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        unimplemented!();
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let new_bytes_pos = if layout.align() > 0 {
            align_up(self.curr_bytes_pos, layout.align()) + layout.size()
        } else {
            self.curr_bytes_pos + layout.size()
        };

        if new_bytes_pos >= self.curr_page_pos {
            return Err(AllocError::MemoryOverlap);
        }

        let ptr = unsafe { NonNull::new_unchecked(self.curr_bytes_pos as *mut u8) };
        self.curr_bytes_pos = new_bytes_pos;
        Ok(ptr)
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        //unimplemented!();
    }

    fn total_bytes(&self) -> usize {
        self.curr_page_pos - self.start_vaddr
    }

    fn used_bytes(&self) -> usize {
        self.curr_bytes_pos - self.start_vaddr
    }

    fn available_bytes(&self) -> usize {
        self.curr_page_pos - self.curr_bytes_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let new_page_pos = if align_pow2 > 0 {
            align_down(self.curr_page_pos, align_pow2) - num_pages * PAGE_SIZE
        } else {
            self.curr_bytes_pos - num_pages * PAGE_SIZE
        };

        //check boundary
        if new_page_pos <= self.curr_bytes_pos {
            return Err(AllocError::MemoryOverlap);
        }
        self.curr_page_pos = new_page_pos;
        Ok(self.used_pages() + num_pages)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {}

    fn total_pages(&self) -> usize {
        let min_page_pos = align_up(self.start_vaddr, PAGE_SIZE);
        (self.page_pos_end - min_page_pos) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.page_pos_end - self.curr_page_pos) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        let bytes_pos_to_page_pos = align_up(self.curr_bytes_pos, PAGE_SIZE);
        (self.curr_bytes_pos - bytes_pos_to_page_pos) / PAGE_SIZE
    }
}

#[inline]
const fn align_down(pos: usize, align: usize) -> usize {
    pos & !(align - 1)
}

#[inline]
const fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}
