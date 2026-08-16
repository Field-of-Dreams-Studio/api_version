use av::{author, panics, safety, ver, verlog};

#[ver(stable, since = "0.2.0")]
pub fn versioned() {}

// Ordering convention: write oldest at TOP, newest (the #[ver]) closest to the fn.
// Rustc processes stacked proc-macro attributes so the innermost (bottom) doc lands
// first in the expanded source — matching this write-order gives newest-first
// rendering in rustdoc, with the #[ver] heading appearing right before the signature.
#[verlog(unstable, since = "0.1.0", note = "Initial implementation", author = "Redstone")]
#[verlog(stable, since = "1.1.0", note = "First stable release", author = "Akari")]
#[ver(update, since = "1.2.0", note = "Added new parameter", author = "Akari")]
pub fn stacked_history(_value: i32) {}

#[safety("caller must ensure ptr is non-null")]
pub unsafe fn safe_annotated(_ptr: *const u8) {}

#[panics("when the input is zero")]
pub fn may_panic(_input: u32) {}

#[panics(never)]
pub fn never_panics() {}

#[author(name = "Redstone", email = "redstone@example.com")]
pub fn authored() {}

fn main() {
    versioned();
    stacked_history(0);
    unsafe { safe_annotated(core::ptr::null()); }
    may_panic(1);
    never_panics();
    authored();
}
