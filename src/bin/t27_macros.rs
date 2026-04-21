//! Topic 27 — Macros: declarative `macro_rules!` and procedural macros (TODO).

macro_rules! say_hi {
    () => {
        println!("hi from macro");
    };
}

fn main() {
    say_hi!();
    println!("TODO: try `vec!`, `println!`, then write a `macro_rules!` with repetition.");
}
