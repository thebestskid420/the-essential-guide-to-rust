//! Topic 10 — Generics, const generics, traits, dyn, advanced traits (extend here).

use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct Article<T: Display> {
    title: T,
}

impl<T: Display> Summary for Article<T> {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

fn announce<T: Summary>(x: &T) {
    println!("{}", x.summarize());
}

fn main() {
    let a = Article { title: "Ownership" };
    announce(&a);

    // Const generic example
    let _: [i32; 3] = [0; 3];
}
