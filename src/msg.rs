//! Message context: caller, self_address, value, origin.

use crate::host;
use crate::types::{Address, U256};

/// Returns the address of the direct caller (msg.sender equivalent).
pub fn caller() -> Address {
    let mut addr = [0u8; 20];
    unsafe { host::caller(addr.as_mut_ptr() as u32) };
    Address(addr)
}

/// Returns the 20-byte contract's own address.
pub fn self_address() -> Address {
    let mut addr = [0u8; 20];
    unsafe { host::self_address(addr.as_mut_ptr() as u32) };
    Address(addr)
}

/// Returns the value (QBIT) sent with this call.
pub fn value() -> U256 {
    let mut val = [0u8; 32];
    unsafe { host::msg_value(val.as_mut_ptr() as u32) };
    U256(val)
}

/// Returns the original transaction sender (tx.origin).
///
/// WARNING: Do NOT use for authorization — vulnerable to phishing via
/// cross-contract calls. Use `caller()` instead.
pub fn origin() -> Address {
    let mut addr = [0u8; 20];
    unsafe { host::tx_origin(addr.as_mut_ptr() as u32) };
    Address(addr)
}
