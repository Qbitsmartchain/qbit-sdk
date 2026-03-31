//! Event emission.

use crate::host;

/// Emits an event with indexed topics and unindexed data.
///
/// # Arguments
/// * `topics` — up to 4 indexed topics, each 32 bytes. The first topic is
///   typically the event signature hash.
/// * `data` — unindexed event data (arbitrary bytes).
pub fn emit(topics: &[[u8; 32]], data: &[u8]) {
    assert!(topics.len() <= 4, "max 4 topics");
    unsafe {
        host::emit_event(
            topics.len() as u32,
            topics.as_ptr() as u32,
            data.as_ptr() as u32,
            data.len() as u32,
        );
    }
}
