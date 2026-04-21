//! Crate-level docs for this binary (`//!` on first lines).
//! Topic 15 — Comments and rustdoc.

/// Adds one (document this function for `cargo doc --open`).
pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // Line comment
    /* block comment */
    println!("{}", add_one(40));
}
