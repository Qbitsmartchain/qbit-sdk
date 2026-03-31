//! Core types for QBit smart contracts: Address, U256, Hash256.

/// A 20-byte account address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// The zero address (used for contract deployment detection).
    pub const ZERO: Self = Address([0u8; 20]);

    /// Creates an address from a byte slice. Panics if len != 20.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(bytes);
        Address(addr)
    }

    /// Returns the raw bytes.
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    /// Returns true if this is the zero address.
    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 20]
    }
}

/// A 256-bit unsigned integer in big-endian byte representation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct U256(pub [u8; 32]);

impl U256 {
    /// The zero value.
    pub const ZERO: Self = U256([0u8; 32]);

    /// Creates a U256 from a u64 value (stored in the low 8 bytes, big-endian).
    pub fn from_u64(val: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[24..32].copy_from_slice(&val.to_be_bytes());
        Self(bytes)
    }

    /// Extracts the low 64 bits. Returns None if the value exceeds u64::MAX.
    pub fn to_u64(&self) -> Option<u64> {
        if self.0[..24].iter().any(|&b| b != 0) {
            return None;
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.0[24..32]);
        Some(u64::from_be_bytes(buf))
    }

    /// Checked addition. Returns None on overflow.
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let mut result = [0u8; 32];
        let mut carry: u16 = 0;
        for i in (0..32).rev() {
            let sum = self.0[i] as u16 + other.0[i] as u16 + carry;
            result[i] = sum as u8;
            carry = sum >> 8;
        }
        if carry != 0 {
            None
        } else {
            Some(U256(result))
        }
    }

    /// Checked subtraction. Returns None on underflow.
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        let mut result = [0u8; 32];
        let mut borrow: i16 = 0;
        for i in (0..32).rev() {
            let diff = self.0[i] as i16 - other.0[i] as i16 - borrow;
            if diff < 0 {
                result[i] = (diff + 256) as u8;
                borrow = 1;
            } else {
                result[i] = diff as u8;
                borrow = 0;
            }
        }
        if borrow != 0 {
            None
        } else {
            Some(U256(result))
        }
    }

    /// Returns true if self >= other.
    pub fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    /// Returns true if self is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

/// A 32-byte hash value.
pub type Hash256 = [u8; 32];
