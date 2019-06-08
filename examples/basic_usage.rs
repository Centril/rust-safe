#![feature(stmt_expr_attributes, proc_macro_hygiene)]

use safe::safe;

fn main() {
    let mut x: u32 = 42;
    let x_ptr = &mut x as *mut u32;

    #[safe(reason = "This is the only reference to x")]
    unsafe {
        x_ptr.write(7);
    }

    #[safe(
        reason = "This is the only reference to x",
        requires = "!x_ptr.is_null()"
    )]
    unsafe {
        x_ptr.write(7);
    }

    let mut allocated: *mut u32;

    #[safe(
        reason = "Malloc will always return a non-null pointer unless there is an out-of-memory error",
        ensures = "!allocated.is_null()"
    )]
    unsafe {
        allocated = libc::malloc(std::mem::size_of::<u32>()) as *mut u32;
    }

    #[safe(
        reason = "The variable has already been allocated",
        requires = "!allocated.is_null()"
    )]
    unsafe {
        *allocated = 42;
    }

    #[safe(
        reason = "The pointer is reset to null after deallocating to prevent use-after-free",
        requires = "!allocated.is_null()",
        ensures = "allocated.is_null()"
    )]
    unsafe {
        libc::free(allocated as *mut libc::c_void);
        allocated = std::ptr::null_mut();
    }
}
