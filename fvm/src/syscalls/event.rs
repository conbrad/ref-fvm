// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use fvm_shared::event::ActorEvent;

use super::Context;
use crate::kernel::Result;
use crate::Kernel;

use fuzzing_tracker::instrument;
#[cfg(feature="tracing")]
// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_custom_probe(line: u64) -> ();
}
// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_syscall_probe(probe: &'static str) -> ();
}

/// Emits an actor event. It takes an DAG-CBOR encoded ActorEvent that has been
/// written to Wasm memory, as an offset and length tuple.
///
/// The FVM validates the structural, syntatic, and semantic correctness of the
/// supplied event, and errors with `IllegalArgument` if the payload was invalid.
///
/// Calling this syscall may immediately halt execution with an out of gas error,
/// if such condition arises.
#[instrument()]
pub fn emit_event(
    context: Context<'_, impl Kernel>,
    event_off: u32, // ActorEvent
    event_len: u32,
) -> Result<()> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.event.emit_event") };
    let evt: ActorEvent = context.memory.read_cbor(event_off, event_len)?;
    context.kernel.emit_event(evt)
}
