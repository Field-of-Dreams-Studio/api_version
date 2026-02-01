//! Example 3: Version history - tracking changes over time
//!
//! This example demonstrates how to use the `ver` attribute to track
//! the evolution of a function through multiple versions. Each version
//! entry is separated by a semicolon, documenting the journey from
//! unstable development to stable release and subsequent updates.

use av::ver;

fn main() {
    println!("Running evolved_function with version history:");
    evolved_function(42, true);
}

#[ver(
    update, since = "1.2.0", note = "Added new parameter", date = "2024-03-01";
    stable, since = "1.0.0", note = "First stable release", date = "2024-02-01";
    unstable, since = "0.1.0", note = "Initial implementation", date = "2024-01-01"
)]
pub fn evolved_function(value: i32, new_param: bool) {
    if new_param {
        println!("New behavior: {}", value);
    } else {
        println!("Original behavior: {}", value);
    }
}

