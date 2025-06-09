// Добавляем простой аллокатор для использования Vec/Box
#![allow(dead_code)]

use core::alloc::{GlobalAlloc, Layout};

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        // Простейший аллокатор, выделяющий статическую память
        static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];
        static mut OFFSET: usize = 0;
        
        let start = (HEAP.as_ptr() as usize + OFFSET + align - 1) & !(align - 1);
        let end = start + size;
        
        if end > HEAP.as_ptr() as usize + HEAP.len() {
            core::ptr::null_mut()
        } else {
            OFFSET = end - HEAP.as_ptr() as usize;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;