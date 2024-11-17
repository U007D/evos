/// Divergent (does not return) function implemented as an infinite loop.  Dummy assignment
/// prevents compiler from eliding function body.
///
/// # Safety
/// `unsafe` required to define the `Reset` handler.
#[allow(unsafe_code)]
#[no_mangle]
pub const unsafe extern "C" fn Reset() -> ! {
    #[allow(clippy::no_effect_underscore_binding)]
        let _x = 42;

    #[allow(clippy::empty_loop)]
    loop {}
}

// The reset vector, a pointer to the reset handler
/// # Safety
/// `unsafe` required to set the `Reset` vector.
#[allow(unsafe_code)]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
