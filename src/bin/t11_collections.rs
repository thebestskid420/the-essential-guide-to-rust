//! Topic 11 — String, Vec, HashMap.

use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("one", 1);
    m.insert("two", 2);

    let mut v = vec![1, 2, 3];
    v.push(4);

    let s = String::from("collections");
    println!("{s}: {m:?} {v:?}");
}
