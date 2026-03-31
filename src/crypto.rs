//! Cryptographic operations: SHA3-256, ML-DSA-65 verify, balance query.

use crate::host;
use crate::types::{Address, Hash256, U256};

/// Computes SHA3-256 hash of data. Domain byte 0x22 is applied by the host.
pub fn sha3(data: &[u8]) -> Hash256 {
    let mut out = [0u8; 32];
    unsafe {
        host::sha3_256(
            data.as_ptr() as u32,
            data.len() as u32,
            out.as_mut_ptr() as u32,
        );
    }
    out
}

/// Verifies an ML-DSA-65 post-quantum signature.
///
/// # Arguments
/// * `pk` — 1,952-byte public key
/// * `msg` — message bytes
/// * `sig` — 3,309-byte signature
///
/// # Returns
/// `true` if the signature is valid, `false` otherwise.
pub fn ml_dsa_verify(pk: &[u8; 1952], msg: &[u8], sig: &[u8; 3309]) -> bool {
    let result = unsafe {
        host::ml_dsa_65_verify(
            pk.as_ptr() as u32,
            msg.as_ptr() as u32,
            msg.len() as u32,
            sig.as_ptr() as u32,
        )
    };
    result == 1
}

/// Returns the QBIT balance of an address.
pub fn balance_of(addr: &Address) -> U256 {
    let mut out = [0u8; 32];
    unsafe {
        host::balance(addr.0.as_ptr() as u32, out.as_mut_ptr() as u32);
    }
    U256(out)
}
