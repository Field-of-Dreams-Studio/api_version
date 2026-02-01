//! Example 1: Minimal usage - only required fields

use av::ver;

#[ver(unstable, since = "0.1.0")]
pub fn minimal_example() {
    println!("This is a minimal example");
}

fn main() {
    minimal_example();
}
