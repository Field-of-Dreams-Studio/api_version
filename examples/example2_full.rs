//! Example 2: Full usage - all fields on one line

use av::ver;

#[ver(deprecated, since = "0.2.0", note = "Use new_function instead", date = "2024-01-15", author = "John Doe")]
pub fn old_function() {
    println!("This function is deprecated");
}

pub fn new_function() {
    println!("Use this instead");
}

fn main() {
    new_function();
}
