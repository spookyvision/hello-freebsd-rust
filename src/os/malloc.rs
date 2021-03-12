use crate::os::kernel::{self, free, malloc, size_t};
use core::alloc::{GlobalAlloc, Layout};
use cty::c_void;
struct KMalloc;

unsafe impl GlobalAlloc for KMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        /* TODO:
        Steven Fackler: You must ensure alignment requirements are matched.
        If the kernel API doesn't offer a malloc variant that takes in an alignment (like posix_memalign),
        you can either reject allocation requests with alignments higher than the allocator's minimum alignment,
        or overallocate and adjust the pointer to an aligned point of the allocation

        the windows implementation of the System allocator does the second if you want something to reference
        */

        /*
        windows allocator source:
        https://github.com/rust-lang/rust/blob/master/library/std/src/sys/windows/alloc.rs#L19
        */
        malloc(
            layout.size() as size_t,
            &mut kernel::M_DEVBUF[0],
            kernel::M_WAITOK as i32,
        ) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr as *mut c_void, &mut kernel::M_DEVBUF[0])
    }
}

#[global_allocator]
static A: KMalloc = KMalloc;

// TODO this is a wart: https://github.com/rust-lang/rust/issues/63348
#[no_mangle]
fn rust_oom() -> ! {
    loop {}
}
