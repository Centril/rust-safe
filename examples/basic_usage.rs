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
        precondition = "!x_ptr.is_null()"
    )]
    unsafe {
        x_ptr.write(7);
    }
}
