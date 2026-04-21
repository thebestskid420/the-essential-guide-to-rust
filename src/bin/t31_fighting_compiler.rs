//! Topic 31 — Fighting the compiler: read errors top to bottom, note `E0xxx` codes, use `rustc --explain`.

fn main() {
    println!("When borrow errors appear:");
    println!("1) Identify which value is borrowed and for how long.");
    println!("2) Check for simultaneous &mut and other aliases.");
    println!("3) Try smaller scopes, cloning, or restructuring data ownership.");
}
