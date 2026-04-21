//! Topic 14 — Modules: this file shows `mod` + `use` in one crate.
//! Compare with `src/bin/` layout: each file is its own binary crate root.

mod inner {
    pub const ID: u32 = 7;

    pub fn greet() -> &'static str {
        "hello from inner"
    }
}

use inner::greet;

fn main() {
    println!("{} (id={})", greet(), inner::ID);
}
