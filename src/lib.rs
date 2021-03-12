#![feature(default_alloc_error_handler)]
#![feature(alloc)]
#![no_std]

mod io;
mod macros;
mod os;

use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};
use cty::{c_char, c_int, c_void};
use os::{kernel, malloc};

extern crate alloc;
use alloc::{string::String, vec::Vec};

struct Hello {}
impl Hello {
    fn new() -> Self {
        Self {}
    }
}

#[no_mangle]
pub extern "C" fn module_event(module: kernel::Module, event: c_int, arg: *mut c_void) -> c_int {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(4);
    println!("hello {:?} event! {}", v, event);
    0
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    println!("crash! {}", panic);
    loop {}
}
