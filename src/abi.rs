//! ABI encoding/decoding and selector computation per QIP-013 Section 6.

use crate::host;
use crate::types::{Address, U256};
use alloc::vec::Vec;

/// Computes the 4-byte function selector.
///
/// selector = SHA3-256(0x22 || signature)[:4]
///
/// Note: QBit uses SHA3-256 (FIPS 202), NOT keccak256 (Ethereum).
/// The domain byte 0x22 (CONTRACT_HASH) is applied by the sha3_256 host function.
pub fn selector(signature: &str) -> [u8; 4] {
    let mut hash = [0u8; 32];
    unsafe {
        host::sha3_256(
            signature.as_ptr() as u32,
            signature.len() as u32,
            hash.as_mut_ptr() as u32,
        );
    }
    let mut sel = [0u8; 4];
    sel.copy_from_slice(&hash[..4]);
    sel
}

/// Trait for types that can be serialized to bytes for storage/ABI.
pub trait Encode {
    /// Encodes the value into bytes, appending to the buffer.
    fn encode_to(&self, buf: &mut Vec<u8>);

    /// Convenience: encode to a new Vec.
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_to(&mut buf);
        buf
    }
}

/// Trait for types that can be deserialized from bytes.
pub trait Decode: Sized {
    /// Decodes from bytes. Returns the value and the number of bytes consumed.
    fn decode(data: &[u8]) -> Option<(Self, usize)>;
}

// --- Standard implementations ---

impl Encode for u8 {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.push(*self);
    }
}

impl Decode for u8 {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        data.first().map(|&b| (b, 1))
    }
}

impl Encode for u16 {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode for u16 {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 2 {
            return None;
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&data[..2]);
        Some((u16::from_be_bytes(bytes), 2))
    }
}

impl Encode for u32 {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode for u32 {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 4 {
            return None;
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data[..4]);
        Some((u32::from_be_bytes(bytes), 4))
    }
}

impl Encode for u64 {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode for u64 {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 8 {
            return None;
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[..8]);
        Some((u64::from_be_bytes(bytes), 8))
    }
}

impl Encode for bool {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.push(if *self { 1 } else { 0 });
    }
}

impl Decode for bool {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        data.first().map(|&b| (b != 0, 1))
    }
}

impl Encode for Address {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }
}

impl Decode for Address {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 20 {
            return None;
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&data[..20]);
        Some((Address(addr), 20))
    }
}

impl Encode for U256 {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }
}

impl Decode for U256 {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 32 {
            return None;
        }
        let mut val = [0u8; 32];
        val.copy_from_slice(&data[..32]);
        Some((U256(val), 32))
    }
}

impl<A: Encode, B: Encode> Encode for (A, B) {
    fn encode_to(&self, buf: &mut Vec<u8>) {
        self.0.encode_to(buf);
        self.1.encode_to(buf);
    }
}

impl<A: Decode, B: Decode> Decode for (A, B) {
    fn decode(data: &[u8]) -> Option<(Self, usize)> {
        let (a, a_len) = A::decode(data)?;
        let (b, b_len) = B::decode(&data[a_len..])?;
        Some(((a, b), a_len + b_len))
    }
}

/// ABI value types for dynamic encoding.
pub enum AbiValue {
    Uint256(U256),
    Address(Address),
    Bool(bool),
    Bytes(Vec<u8>),
}

/// ABI type descriptors.
#[derive(Clone, Copy)]
pub enum AbiType {
    Uint256,
    Address,
    Bool,
    Bytes,
}

/// ABI-encodes a list of 32-byte padded values for contract calls.
///
/// Each value is padded to exactly 32 bytes following Solidity ABI rules:
/// - Integers: left-padded with zeros
/// - Addresses: left-padded with 12 zero bytes
/// - Bytes: right-padded with zeros
pub fn abi_encode(values: &[AbiValue]) -> Vec<u8> {
    let mut buf = Vec::new();
    for value in values {
        let mut word = [0u8; 32];
        match value {
            AbiValue::Uint256(v) => {
                word = v.0;
            }
            AbiValue::Address(a) => {
                // Left-pad: address goes into bytes 12..32
                word[12..32].copy_from_slice(&a.0);
            }
            AbiValue::Bool(b) => {
                word[31] = if *b { 1 } else { 0 };
            }
            AbiValue::Bytes(data) => {
                // For simplicity, only encode up to 32 bytes inline
                let len = data.len().min(32);
                word[..len].copy_from_slice(&data[..len]);
            }
        }
        buf.extend_from_slice(&word);
    }
    buf
}

/// ABI-decodes a series of 32-byte values.
pub fn abi_decode(data: &[u8], types: &[AbiType]) -> Option<Vec<AbiValue>> {
    if data.len() < types.len() * 32 {
        return None;
    }
    let mut values = Vec::new();
    for (i, ty) in types.iter().enumerate() {
        let offset = i * 32;
        let word = &data[offset..offset + 32];
        let value = match ty {
            AbiType::Uint256 => {
                let mut val = [0u8; 32];
                val.copy_from_slice(word);
                AbiValue::Uint256(U256(val))
            }
            AbiType::Address => {
                let mut addr = [0u8; 20];
                addr.copy_from_slice(&word[12..32]);
                AbiValue::Address(Address(addr))
            }
            AbiType::Bool => AbiValue::Bool(word[31] != 0),
            AbiType::Bytes => AbiValue::Bytes(word.to_vec()),
        };
        values.push(value);
    }
    Some(values)
}
