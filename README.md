# qbit-sdk

Rust SDK for writing smart contracts on the [QBit SmartChain](https://github.com/Qbitsmartchain/qbitchain).

## Overview

`qbit-sdk` provides type-safe wrappers around the 25 QVM host functions, a storage abstraction layer, ABI encoding/decoding, and entry point helpers. The SDK compiles to RISC-V (`riscv32emac-unknown-none-polkavm`) and runs inside the PolkaVM guest — it has zero host-side dependencies.

## Features

- **Post-quantum crypto**: ML-DSA-65 signature verification, ML-KEM-768 encapsulation, Poseidon2 hashing
- **Storage abstractions**: `StorageValue<T>` and `StorageMap<K, V>` with automatic slot derivation
- **ABI encoding**: Solidity-compatible 32-byte padded encoding with SHA3-256 selectors
- **Cross-contract calls**: `call`, `delegate_call`, `static_call`
- **Events**: Indexed topic emission
- **`no_std`**: Bump allocator, no filesystem/networking/threads

## Quick Start

```rust
#![no_std]
#![no_main]

extern crate alloc;

#[global_allocator]
static ALLOCATOR: qbit_sdk::BumpAllocator = qbit_sdk::BumpAllocator::new();

use qbit_sdk::prelude::*;

#[polkavm_derive::polkavm_export]
extern "C" fn call(input_ptr: u32, input_len: u32) {
    let input = unsafe { entry::read_input(input_ptr, input_len) };
    // Contract logic here
    entry::return_success();
}
```

## Building

```bash
cargo +nightly build --release \
    --target $(polkatool get-target-json-path --bitness 32) \
    -Z build-std="core,alloc" \
    -Z build-std-features="panic_immediate_abort"

polkatool link target/riscv32emac-unknown-none-polkavm/release/<name> -o <name>.qvm
```

## Documentation

See [QIP-013: QBit SDK Specification](https://github.com/Qbitsmartchain/qbitchain/blob/main/docs/QIP-013-qbit-sdk-specification.md) for the full specification.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
