//! Topic 16 — println!, format!, Debug, Display, formatting options.

use std::fmt;

struct Id(u32);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

fn main() {
    let id = Id(9);
    println!("Display: {id}");
    println!("Debug: {:?}", (1, 2));
    println!("aligned: {:>5}", 3);
    println!("{}", format!("pi~{:.2}", 3.1415));
}
