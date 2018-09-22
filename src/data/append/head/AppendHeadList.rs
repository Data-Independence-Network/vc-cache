use core::mem;
use core::ptr::{self, NonNull, Unique};
use alloc::alloc::{Alloc, Layout, Global, handle_alloc_error};

static mut memoryAllocator: Global = Global;

#[repr(C)]
pub struct AppendHeadList {
    buf: Unique<u32>,
    cap: u8,
    len: u8,
}

impl AppendHeadList {

    pub fn create(data: &[u32]) -> AppendHeadList {

    }

    pub fn append(&self, data: &[u32]) {

    }

    pub fn reset(&self, data: &[u32]) {
        
    }

    pub fn new(cap: u8) -> AppendHeadList {
        AppendHeadList {
            buf: [],
            cap,
            len: 0,
        }
    }
    fn allocate_in(cap: usize) -> u32 {
        unsafe {
            let shifted: u32 = 256;
            let shiftTest = shifted >> 4;
            let shiftTest2 = shifted << 4;
            // Allocate 4 its worth
            let layout = Layout::from_size_align_unchecked(16, 4);
            let result = memoryAllocator.alloc(layout);
            match result {
                Ok(ptr) => ptr.cast(),
                Err(_) => handle_alloc_error(layout),
            }
        }
    }
}