//! Block context: block_number, block_timestamp, chain_id, gas_remaining.

// Inline asm reads RV32 A1 register — sub-register warnings are x86-only noise.
#![allow(asm_sub_register)]

use crate::host;

/// Returns the current block number as u64.
///
/// On RV32EM, the host returns low 32 bits in A0 and high 32 bits in A1.
pub fn number() -> u64 {
    let low = unsafe { host::block_number() } as u64;
    let high: u32;
    unsafe { core::arch::asm!("mv {0}, a1", out(reg) high) };
    ((high as u64) << 32) | low
}

/// Returns the current block timestamp in milliseconds since Unix epoch.
pub fn timestamp() -> u64 {
    let low = unsafe { host::block_timestamp() } as u64;
    let high: u32;
    unsafe { core::arch::asm!("mv {0}, a1", out(reg) high) };
    ((high as u64) << 32) | low
}

/// Returns the chain ID (0 = QBit native, >0 = QChain).
pub fn chain_id() -> u16 {
    unsafe { host::chain_id() as u16 }
}

/// Returns the remaining gas for this call frame.
pub fn gas_remaining() -> u64 {
    let low = unsafe { host::gas_remaining() } as u64;
    let high: u32;
    unsafe { core::arch::asm!("mv {0}, a1", out(reg) high) };
    ((high as u64) << 32) | low
}
