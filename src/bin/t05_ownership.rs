//! Topic 5 — Ownership and borrowing.

fn takes_ownership(s: String) {
    println!("owned: {s}");
}

fn borrows(s: &str) {
    println!("borrowed: {s}");
}

fn main() {
    let s = String::from("rust");
    borrows(&s);
    takes_ownership(s);
    // s moved; borrow examples with mut in your own exercises
}
