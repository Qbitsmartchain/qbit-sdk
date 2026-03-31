//! QBit SDK — Rust library for writing smart contracts on the QBit blockchain.
//!
//! Provides type-safe wrappers around the 25 QVM host functions, storage
//! abstractions, ABI encoding/decoding, and entry point helpers.
//!
//! See [QIP-013](https://github.com/Qbitsmartchain/qbitchain/blob/main/docs/QIP-013-qbit-sdk-specification.md)
//! for the full specification.

#![no_std]

extern crate alloc;

pub mod abi;
pub mod block;
pub mod call;
pub mod crypto;
pub mod entry;
pub mod error;
pub mod event;
pub mod host;
pub mod msg;
pub mod storage;
pub mod types;

/// Prelude — import everything a contract typically needs.
pub mod prelude {
    pub use crate::block;
    pub use crate::call;
    pub use crate::crypto;
    pub use crate::entry;
    pub use crate::error;
    pub use crate::event;
    pub use crate::msg;
    pub use crate::storage;
    pub use crate::types::{Address, Hash256, U256};
    pub use crate::abi::{Decode, Encode};
    pub use polkavm_derive::{polkavm_export, polkavm_import};
}

/// Default heap size for the bump allocator: 64 KB.
pub const HEAP_SIZE: usize = 64 * 1024;

/// Simple bump allocator for `no_std` contracts.
///
/// Allocates from a fixed-size static buffer. Does not support deallocation —
/// all memory is freed when the contract execution terminates.
pub struct BumpAllocator {
    heap: core::cell::UnsafeCell<[u8; HEAP_SIZE]>,
    offset: core::sync::atomic::AtomicUsize,
}

// SAFETY: The allocator is only used in a single-threaded guest VM environment.
unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    /// Creates a new bump allocator with a zeroed heap.
    pub const fn new() -> Self {
        Self {
            heap: core::cell::UnsafeCell::new([0u8; HEAP_SIZE]),
            offset: core::sync::atomic::AtomicUsize::new(0),
        }
    }
}

unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let offset = self.offset.load(core::sync::atomic::Ordering::Relaxed);
        let aligned = (offset + align - 1) & !(align - 1);
        let new_offset = aligned + size;
        if new_offset > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        self.offset
            .store(new_offset, core::sync::atomic::Ordering::Relaxed);
        (self.heap.get() as *mut u8).add(aligned)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Bump allocator does not reclaim memory.
    }
}

/// Panic handler for `no_std` guest environment.
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::arch::asm!("unimp", options(noreturn)) }
}
