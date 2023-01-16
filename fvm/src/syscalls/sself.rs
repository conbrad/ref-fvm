// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use anyhow::Context as _;
use fvm_shared::sys;

use super::Context;
use crate::kernel::{ClassifyResult, Kernel, Result};

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
/// Returns the root CID of the actor's state by writing it in the specified buffer.
///
/// The returned u32 represents the _actual_ length of the CID. If the supplied
/// buffer is smaller, no value will have been written. The caller must retry
/// with a larger buffer.
#[instrument()]
pub fn root(context: Context<'_, impl Kernel>, obuf_off: u32, obuf_len: u32) -> Result<u32> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.self.root") };
    context.memory.check_bounds(obuf_off, obuf_len)?;

    let root = context.kernel.root()?;

    context.memory.write_cid(&root, obuf_off, obuf_len)
}

#[instrument()]
pub fn set_root(context: Context<'_, impl Kernel>, cid_off: u32) -> Result<()> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.self.set_root") };
    let cid = context.memory.read_cid(cid_off)?;
    context.kernel.set_root(cid)?;
    Ok(())
}

#[instrument()]
pub fn current_balance(context: Context<'_, impl Kernel>) -> Result<sys::TokenAmount> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.self.current_balance") };
    let balance = context.kernel.current_balance()?;
    balance
        .try_into()
        .context("balance exceeds u128")
        .or_fatal()
}

#[instrument()]
pub fn self_destruct(
    context: Context<'_, impl Kernel>,
    addr_off: u32,
    addr_len: u32,
) -> Result<()> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.self.self_destruct") };
    let addr = context.memory.read_address(addr_off, addr_len)?;
    context.kernel.self_destruct(&addr)?;
    Ok(())
}
