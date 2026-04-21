//! Topic 17 — Lifetimes (elision, explicit, `'static` — extend with harder examples).

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn static_str() -> &'static str {
    "program text"
}

fn main() {
    let s = longest("short", "loooooong");
    println!("{s}, {}", static_str());
}
