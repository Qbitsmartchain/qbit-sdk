//! Cross-contract call wrappers.

// Result<_, ()> is intentional — cross-contract calls have no meaningful error
// payload beyond success/failure. A custom error type would add complexity
// with no benefit since the host only returns 0 or 1.
#![allow(clippy::result_unit_err)]

use crate::host;
use crate::types::Address;
use alloc::vec::Vec;

/// Maximum return data buffer size.
const MAX_OUTPUT: usize = 4096;

/// Calls another contract.
///
/// # Arguments
/// * `to` — target contract address
/// * `input` — calldata (4-byte selector + ABI-encoded args)
/// * `gas` — gas to forward (0 = forward maximum allowed)
///
/// # Returns
/// `Ok(output_bytes)` on success, `Err(())` on failure.
pub fn call(to: &Address, input: &[u8], gas: u64) -> Result<Vec<u8>, ()> {
    let mut output = [0u8; MAX_OUTPUT];
    let result = unsafe {
        host::call_contract(
            to.0.as_ptr() as u32,
            input.as_ptr() as u32,
            input.len() as u32,
            gas as u32,
            output.as_mut_ptr() as u32,
            output.len() as u32,
        )
    };
    if result == 1 {
        Ok(output.to_vec())
    } else {
        Err(())
    }
}

/// Delegate call — callee runs with this contract's storage and caller.
pub fn delegate(to: &Address, input: &[u8], gas: u64) -> Result<Vec<u8>, ()> {
    let mut output = [0u8; MAX_OUTPUT];
    let result = unsafe {
        host::delegate_call(
            to.0.as_ptr() as u32,
            input.as_ptr() as u32,
            input.len() as u32,
            gas as u32,
            output.as_mut_ptr() as u32,
            output.len() as u32,
        )
    };
    if result == 1 {
        Ok(output.to_vec())
    } else {
        Err(())
    }
}

/// Static call — read-only, no state modifications allowed.
pub fn static_call(to: &Address, input: &[u8], gas: u64) -> Result<Vec<u8>, ()> {
    let mut output = [0u8; MAX_OUTPUT];
    let result = unsafe {
        host::static_call(
            to.0.as_ptr() as u32,
            input.as_ptr() as u32,
            input.len() as u32,
            gas as u32,
            output.as_mut_ptr() as u32,
            output.len() as u32,
        )
    };
    if result == 1 {
        Ok(output.to_vec())
    } else {
        Err(())
    }
}
