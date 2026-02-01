//! Example 5: Testing deprecated warning
//!
//! This will ALWAYS show a deprecation warning (regardless of features):
//! ```bash
//! cargo run --example example5_warning_deprecated
//! ```

use av::ver;

#[ver(deprecated, since = "0.2.0", note = "Use better_function instead")]
pub fn legacy_function(x: i32) -> i32 {
    x * 2
}

#[ver(stable, since = "0.2.0", note = "Replacement for legacy_function")]
pub fn better_function(x: i32) -> i32 {
    x * 2
}

fn main() {
    // This should ALWAYS trigger a deprecation warning
    let result1 = legacy_function(5);
    println!("Legacy result: {}", result1);

    // This should NOT trigger any warning
    let result2 = better_function(5);
    println!("Better result: {}", result2);
}
