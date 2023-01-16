// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use fvm_shared::randomness::RANDOMNESS_LENGTH;

use super::Context;
use crate::kernel::Result;
use crate::Kernel;

// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_syscall_probe(probe: &'static str) -> ();
}
use fuzzing_tracker::instrument;
#[cfg(feature="tracing")]
// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_custom_probe(line: u64) -> ();
}

/// Gets 32 bytes of randomness from the ticket chain.
/// The supplied output buffer must have at least 32 bytes of capacity.
/// If this syscall succeeds, exactly 32 bytes will be written starting at the
/// supplied offset.
#[instrument()]
pub fn get_chain_randomness(
    context: Context<'_, impl Kernel>,
    pers: i64,  // DomainSeparationTag
    round: i64, // ChainEpoch
    entropy_off: u32,
    entropy_len: u32,
) -> Result<[u8; RANDOMNESS_LENGTH]> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.rand.get_chain_randomness") };
    let entropy = context.memory.try_slice(entropy_off, entropy_len)?;
    context
        .kernel
        .get_randomness_from_tickets(pers, round, entropy)
}

/// Gets 32 bytes of randomness from the beacon system (currently Drand).
/// The supplied output buffer must have at least 32 bytes of capacity.
/// If this syscall succeeds, exactly 32 bytes will be written starting at the
/// supplied offset.
#[instrument()]
pub fn get_beacon_randomness(
    context: Context<'_, impl Kernel>,
    pers: i64,  // DomainSeparationTag
    round: i64, // ChainEpoch
    entropy_off: u32,
    entropy_len: u32,
) -> Result<[u8; RANDOMNESS_LENGTH]> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.rand.get_beacon_randomness") };
    let entropy = context.memory.try_slice(entropy_off, entropy_len)?;
    context
        .kernel
        .get_randomness_from_beacon(pers, round, entropy)
}
