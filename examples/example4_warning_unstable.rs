//! Example 4: Testing unstable warning
//!
//! To test WITH warnings (deprecated_for_unstable enabled):
//! ```bash
//! cargo run --example example4_warning_unstable --features deprecated_for_unstable
//! ```
//!
//! To test WITHOUT warnings (default):
//! ```bash
//! cargo run --example example4_warning_unstable
//! ```

use av::ver;

#[ver(unstable, since = "0.1.0", note = "This API may change")]
pub fn experimental_feature() {
    println!("⚠️  This is an experimental feature");
}

fn main() {
    // This should trigger a deprecation warning when deprecated_for_unstable is enabled
    experimental_feature();
}
