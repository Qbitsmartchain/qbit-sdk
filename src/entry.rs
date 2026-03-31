//! Entry point helpers: read_input, return_success, return_error.

/// Reads the calldata passed by the host.
///
/// The host writes calldata at address 0x10000 and passes
/// (ptr, len) as arguments to the entry point function.
///
/// # Safety
/// The caller must ensure `ptr` and `len` are the values passed by the host
/// as function arguments to the `deploy` or `call` export.
pub unsafe fn read_input(ptr: u32, len: u32) -> &'static [u8] {
    core::slice::from_raw_parts(ptr as *const u8, len as usize)
}

/// Returns success to the host with no output data (exit code 0).
pub fn return_success() -> ! {
    unsafe { core::arch::asm!("li a0, 0", "ret", options(noreturn)) }
}

/// Returns failure to the host (exit code 1).
pub fn return_error() -> ! {
    unsafe { core::arch::asm!("li a0, 1", "ret", options(noreturn)) }
}
