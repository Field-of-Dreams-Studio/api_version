//! Complete example — one unsafe function annotated with every macro in the suite.
//!
//! **Ordering.** Rust processes stacked proc-macro attributes such that the
//! attribute closest to the `fn` renders FIRST in rustdoc. This example is written
//! to render, top to bottom in the generated docs:
//!
//! 1. `## Safety`     — critical for callers of an `unsafe fn`, so put first
//! 2. `## Panics`     — next-most important runtime concern
//! 3. `### Update Version: 1.3.0`  — the current version, highlighted
//! 4. `Version: 1.2.0, **Update**` — historical (newest of the log)
//! 5. `Version: 1.0.0, **Stable**`
//! 6. `Version: 0.1.0, **Unstable**` — historical (oldest of the log)
//! 7. `## Authors`    — attribution, least urgent for a caller
//!
//! To get this reader-facing order, write the attributes in reverse: what should
//! appear LAST in the docs goes at the TOP of the attribute stack.

use av::{author, panics, safety, ver, verlog};

/// Copy `count` bytes from `src` to `dst`.
///
/// This function is a thin wrapper over `core::ptr::copy_nonoverlapping`.
#[author(name = "Redstone", email = "redstone@example.com", role = "maintainer")]
#[verlog(unstable, since = "0.1.0", note = "Initial API")]
#[verlog(stable, since = "1.0.0", note = "Stabilised after review")]
#[verlog(update, since = "1.2.0", note = "Added a debug-build bounds check")]
#[ver(update, since = "1.3.0", note = "Renamed parameter for clarity", date = "2026-08-16")]
#[panics(
    "on debug builds when count exceeds 1 << 30",
    "if the source or destination pointer is null"
)]
#[safety(
    "src is a valid pointer to at least count readable bytes",
    "dst is a valid pointer to at least count writable bytes",
    "src and dst regions do not overlap"
)]
pub unsafe fn copy_bytes(src: *const u8, dst: *mut u8, count: usize) {
    core::ptr::copy_nonoverlapping(src, dst, count);
}

fn main() {
    let src = [1u8, 2, 3, 4];
    let mut dst = [0u8; 4];
    unsafe { copy_bytes(src.as_ptr(), dst.as_mut_ptr(), src.len()); }
    println!("{dst:?}");
}
