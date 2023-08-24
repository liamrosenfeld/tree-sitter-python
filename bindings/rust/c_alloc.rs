use std::alloc::{self, Layout};

#[no_mangle]
pub unsafe extern "C" fn calloc(count: usize, size: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(count * size, std::mem::align_of::<u8>());
    alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(new_size, std::mem::align_of::<u8>());
    alloc::realloc(ptr, layout, new_size)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    let layout = Layout::from_size_align_unchecked(0, std::mem::align_of::<u8>());
    alloc::dealloc(ptr, layout);
}
