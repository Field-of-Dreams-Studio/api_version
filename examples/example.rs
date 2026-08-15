use av::{author, panics, safety, ver};

#[ver(stable, since = "0.2.0")]
pub fn versioned() {}

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
    unsafe { safe_annotated(core::ptr::null()); }
    may_panic(1);
    never_panics();
    authored();
}
