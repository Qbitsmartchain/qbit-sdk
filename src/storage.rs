//! Storage abstraction: raw access, StorageValue<T>, StorageMap<K, V>.

use crate::abi::{Decode, Encode};
use crate::host;
use alloc::vec::Vec;
use core::marker::PhantomData;

/// Reads a raw 32-byte value from a 32-byte storage key.
pub fn raw_get(key: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe { host::storage_read(key.as_ptr() as u32, out.as_mut_ptr() as u32) };
    out
}

/// Writes a raw 32-byte value to a 32-byte storage key.
pub fn raw_set(key: &[u8; 32], value: &[u8; 32]) {
    unsafe { host::storage_write(key.as_ptr() as u32, value.as_ptr() as u32) };
}

/// Removes a storage entry by key.
pub fn raw_remove(key: &[u8; 32]) {
    unsafe { host::storage_remove(key.as_ptr() as u32) };
}

/// Computes a storage slot key from a namespace and sub-key.
///
/// slot = SHA3-256(0x22 || namespace || sub_key)
///
/// The domain byte 0x22 is applied by the sha3_256 host function.
pub fn derive_slot(namespace: &[u8], sub_key: &[u8]) -> [u8; 32] {
    let mut input = Vec::with_capacity(namespace.len() + sub_key.len());
    input.extend_from_slice(namespace);
    input.extend_from_slice(sub_key);
    let mut out = [0u8; 32];
    unsafe {
        host::sha3_256(
            input.as_ptr() as u32,
            input.len() as u32,
            out.as_mut_ptr() as u32,
        );
    }
    out
}

/// A single typed value stored at a fixed storage slot.
///
/// The slot is derived from the namespace: SHA3-256(0x22 || namespace).
pub struct StorageValue<T> {
    namespace: &'static [u8],
    _marker: PhantomData<T>,
}

impl<T: Encode + Decode> StorageValue<T> {
    /// Creates a new StorageValue with the given namespace.
    pub const fn new(namespace: &'static [u8]) -> Self {
        Self {
            namespace,
            _marker: PhantomData,
        }
    }

    fn slot(&self) -> [u8; 32] {
        derive_slot(self.namespace, &[])
    }

    /// Reads the value from storage. Returns decoded zero bytes if not set.
    pub fn get(&self) -> T {
        let raw = raw_get(&self.slot());
        match T::decode(&raw) {
            Some((val, _)) => val,
            None => match T::decode(&[0u8; 32]) {
                Some((val, _)) => val,
                None => unsafe { core::hint::unreachable_unchecked() },
            },
        }
    }

    /// Writes the value to storage.
    pub fn set(&self, value: &T) {
        let encoded = value.encode();
        let mut raw = [0u8; 32];
        let len = encoded.len().min(32);
        // Right-align for numeric types (big-endian U256 convention)
        raw[32 - len..].copy_from_slice(&encoded[..len]);
        raw_set(&self.slot(), &raw);
    }

    /// Removes the value from storage (resets to zero bytes).
    pub fn clear(&self) {
        raw_remove(&self.slot());
    }
}

/// A key-value mapping in contract storage.
///
/// Each entry's slot is derived from: SHA3-256(0x22 || namespace || encode(key)).
pub struct StorageMap<K, V> {
    namespace: &'static [u8],
    _marker: PhantomData<(K, V)>,
}

impl<K: Encode, V: Encode + Decode> StorageMap<K, V> {
    /// Creates a new StorageMap with the given namespace.
    pub const fn new(namespace: &'static [u8]) -> Self {
        Self {
            namespace,
            _marker: PhantomData,
        }
    }

    fn slot_for(&self, key: &K) -> [u8; 32] {
        let key_bytes = key.encode();
        derive_slot(self.namespace, &key_bytes)
    }

    /// Reads the value for key. Returns default if not set.
    pub fn get(&self, key: &K) -> V {
        let raw = raw_get(&self.slot_for(key));
        match V::decode(&raw) {
            Some((val, _)) => val,
            None => match V::decode(&[0u8; 32]) {
                Some((val, _)) => val,
                None => unsafe { core::hint::unreachable_unchecked() },
            },
        }
    }

    /// Sets the value for key.
    pub fn insert(&self, key: &K, value: &V) {
        let encoded = value.encode();
        let mut raw = [0u8; 32];
        let len = encoded.len().min(32);
        raw[32 - len..].copy_from_slice(&encoded[..len]);
        raw_set(&self.slot_for(key), &raw);
    }

    /// Removes the entry for key.
    pub fn remove(&self, key: &K) {
        raw_remove(&self.slot_for(key));
    }

    /// Returns true if the key has a non-zero value.
    pub fn contains(&self, key: &K) -> bool {
        let raw = raw_get(&self.slot_for(key));
        raw != [0u8; 32]
    }
}
