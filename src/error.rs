//! Error handling: revert and require.

/// Reverts the contract execution with a reason string.
///
/// The contract returns a non-zero exit code to the host.
/// Gas consumed up to this point is NOT refunded.
pub fn revert(_reason: &str) -> ! {
    unsafe {
        core::arch::asm!("li a0, 1", "ret", options(noreturn));
    }
}

/// Checks a condition and reverts if false.
///
/// # Example
/// ```ignore
/// require(sender_balance.ge(&amount), "insufficient balance");
/// ```
pub fn require(condition: bool, reason: &str) {
    if !condition {
        revert(reason);
    }
}
