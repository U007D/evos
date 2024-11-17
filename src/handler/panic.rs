use core::panic::PanicInfo;

/// Divergent (does not return) function implemented as an infinite loop.  Dummy assignment
/// prevents compiler from eliding function body.
#[panic_handler]
pub const fn panic(_panic: &PanicInfo<'_>) -> ! {
    #[allow(clippy::no_effect_underscore_binding)]
    let _x = 42;

    #[allow(clippy::empty_loop)]
    loop {}
}
