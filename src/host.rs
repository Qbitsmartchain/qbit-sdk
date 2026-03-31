//! Raw host function imports matching QVM dispatch_syscall() names exactly.
//!
//! These are the 25 host functions provided by the QVM. Each function name
//! must match the string used in `dispatch_syscall()` in `qbit-vm/src/host.rs`.
//! Safe wrappers are provided in sibling modules (storage, msg, block, etc.).

#[polkavm_derive::polkavm_import]
extern "C" {
    // --- Storage ---

    /// Reads a 32-byte value from storage by 32-byte key.
    /// key_ptr: pointer to 32-byte key. out_ptr: pointer to 32-byte output buffer.
    /// Returns 1 on success, 0 if key not found.
    pub fn storage_read(key_ptr: u32, out_ptr: u32) -> u32;

    /// Writes a 32-byte value to storage by 32-byte key.
    /// key_ptr: pointer to 32-byte key. val_ptr: pointer to 32-byte value.
    /// Returns 1 on success, 0 on failure (e.g., read_only context).
    pub fn storage_write(key_ptr: u32, val_ptr: u32) -> u32;

    /// Removes a storage entry by 32-byte key.
    /// Returns 1 on success, 0 on failure.
    pub fn storage_remove(key_ptr: u32) -> u32;

    // --- Crypto ---

    /// SHA3-256 hash. Domain byte 0x22 (CONTRACT_HASH) is applied by the host.
    /// data_ptr/data_len: input bytes. out_ptr: 32-byte output buffer.
    /// Returns 1 on success.
    pub fn sha3_256(data_ptr: u32, data_len: u32, out_ptr: u32) -> u32;

    /// Poseidon2 hash over Goldilocks field elements. Domain byte 0x23.
    /// input_ptr: array of 8-byte LE field elements. n_elements: count.
    /// out_ptr: 8-byte LE field element output.
    /// Returns 1 on success.
    pub fn poseidon2_hash(input_ptr: u32, n_elements: u32, out_ptr: u32) -> u32;

    /// ML-DSA-65 post-quantum signature verification.
    /// pk_ptr: 1,952-byte public key. msg_ptr/msg_len: message bytes.
    /// sig_ptr: 3,309-byte signature.
    /// Returns 1 if valid, 0 if invalid.
    pub fn ml_dsa_65_verify(pk_ptr: u32, msg_ptr: u32, msg_len: u32, sig_ptr: u32) -> u32;

    /// ML-KEM-768 encapsulation.
    /// ek_ptr: 1,184-byte encapsulation key.
    /// ct_out_ptr: 1,088-byte ciphertext output.
    /// ss_out_ptr: 32-byte shared secret output.
    /// Returns 1 on success.
    pub fn ml_kem_768_encapsulate(ek_ptr: u32, ct_out_ptr: u32, ss_out_ptr: u32) -> u32;

    /// ML-KEM-768 decapsulation (committee context only).
    /// ct_ptr: 1,088-byte ciphertext. ss_out_ptr: 32-byte shared secret output.
    /// Returns 1 on success.
    pub fn ml_kem_768_decapsulate(ct_ptr: u32, ss_out_ptr: u32) -> u32;

    /// Poseidon2 Merkle proof verification.
    /// root_ptr: 8-byte LE field element (Merkle root).
    /// leaf_ptr: 8-byte LE field element (leaf value).
    /// proof_ptr: array of 8-byte LE field elements (sibling hashes).
    /// proof_len: number of proof elements.
    /// Returns 1 if proof is valid, 0 otherwise.
    pub fn poseidon2_merkle_verify(
        root_ptr: u32,
        leaf_ptr: u32,
        proof_ptr: u32,
        proof_len: u32,
    ) -> u32;

    // --- Context ---

    /// Writes 20-byte caller address (msg.sender) to out_ptr.
    /// Returns 1 on success.
    pub fn caller(out_ptr: u32) -> u32;

    /// Writes 20-byte contract's own address to out_ptr.
    /// Returns 1 on success.
    pub fn self_address(out_ptr: u32) -> u32;

    /// Writes 32-byte msg.value (U256, big-endian) to out_ptr.
    /// Returns 1 on success.
    pub fn msg_value(out_ptr: u32) -> u32;

    /// Returns current block number. Low 32 bits in A0, high 32 bits in A1.
    pub fn block_number() -> u32;

    /// Returns current block timestamp (ms since epoch).
    /// Low 32 bits in A0, high 32 bits in A1.
    pub fn block_timestamp() -> u32;

    /// Returns chain ID (u16 fits in u32). 0 = QBit native, >0 = QChain.
    pub fn chain_id() -> u32;

    /// Returns remaining gas for this call frame.
    /// Low 32 bits in A0, high 32 bits in A1.
    pub fn gas_remaining() -> u32;

    /// Writes 20-byte tx.origin address to out_ptr.
    /// WARNING: Do NOT use for authorization — vulnerable to phishing.
    /// Returns 1 on success.
    pub fn tx_origin(out_ptr: u32) -> u32;

    // --- Balance ---

    /// Reads 32-byte balance (U256) of address at addr_ptr into out_ptr.
    /// Returns 1 on success.
    pub fn balance(addr_ptr: u32, out_ptr: u32) -> u32;

    // --- Events ---

    /// Emits an event with indexed topics and unindexed data.
    /// topic_count: number of topics (max 4). Each topic is 32 bytes.
    /// topics_ptr: pointer to contiguous topic data.
    /// data_ptr/data_len: unindexed event data.
    /// Returns 1 on success.
    pub fn emit_event(
        topic_count: u32,
        topics_ptr: u32,
        data_ptr: u32,
        data_len: u32,
    ) -> u32;

    // --- Cross-contract calls ---

    /// Calls another contract.
    /// addr_ptr: 20-byte target address.
    /// input_ptr/input_len: calldata (4-byte selector + ABI-encoded args).
    /// gas: gas to forward. out_ptr/out_max: output buffer.
    /// Returns 1 on success, 0 on failure.
    pub fn call_contract(
        addr_ptr: u32,
        input_ptr: u32,
        input_len: u32,
        gas: u32,
        out_ptr: u32,
        out_max: u32,
    ) -> u32;

    /// Delegate call — callee runs with caller's storage and msg.sender.
    pub fn delegate_call(
        addr_ptr: u32,
        input_ptr: u32,
        input_len: u32,
        gas: u32,
        out_ptr: u32,
        out_max: u32,
    ) -> u32;

    /// Static call — read-only, no state modifications allowed.
    pub fn static_call(
        addr_ptr: u32,
        input_ptr: u32,
        input_len: u32,
        gas: u32,
        out_ptr: u32,
        out_max: u32,
    ) -> u32;

    // --- Contract lifecycle ---

    /// Creates a new contract.
    /// code_ptr/code_len: bytecode. salt_ptr: 32-byte CREATE2 salt.
    /// value_ptr: 32-byte U256 value to send. out_addr_ptr: 20-byte output address.
    /// Returns 1 on success.
    pub fn create_contract(
        code_ptr: u32,
        code_len: u32,
        salt_ptr: u32,
        value_ptr: u32,
        out_addr_ptr: u32,
    ) -> u32;

    /// Reads 32-byte code hash of address at addr_ptr into out_ptr.
    /// Returns 1 on success, 0 if address has no code.
    pub fn code_hash(addr_ptr: u32, out_ptr: u32) -> u32;

    /// Returns code size in bytes for address at addr_ptr.
    pub fn code_size(addr_ptr: u32) -> u32;
}
